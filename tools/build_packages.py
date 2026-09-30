"""Generate both public packages from Bazel's canonical proto sources."""

import argparse
import gzip
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import tarfile

from grpc_tools import protoc
from hatchling.build import build_sdist, build_wheel


def compile_proto(arguments: list[str]) -> None:
    status = protoc.main(["protoc", *arguments])
    if status:
        raise RuntimeError(f"Protobuf generation failed with exit code {status}")


def archive_npm(package: Path, destination: Path) -> None:
    # npm accepts a gzip tar archive with a package/ root. Fix metadata so the
    # archive is reproducible and does not contain local user or path information.
    with destination.open("wb") as stream:
        with gzip.GzipFile(
            filename="", mode="wb", fileobj=stream, mtime=0
        ) as compressed:
            with tarfile.open(fileobj=compressed, mode="w") as archive:
                for path in sorted(package.rglob("*")):
                    if not path.is_file():
                        continue
                    info = archive.gettarinfo(
                        path, arcname=f"package/{path.relative_to(package)}"
                    )
                    info.uid = info.gid = info.mtime = 0
                    info.uname = info.gname = ""
                    info.mode = 0o644
                    with path.open("rb") as contents:
                        archive.addfile(info, contents)


def build(arguments: argparse.Namespace) -> None:
    schemas = sorted(arguments.schema)
    templates = arguments.templates.absolute()
    plugin = arguments.plugin.absolute()
    output = arguments.output.absolute()
    version = arguments.version.read_text().strip()
    if not re.fullmatch(r"(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)", version):
        raise ValueError("VERSION must contain a stable major.minor.patch version")

    python = output / "python"
    javascript = output / "javascript"
    distributions = output / "dist"
    distributions.mkdir(parents=True)
    asyncapis = [path.absolute() for path in arguments.asyncapi]
    schema_directory = arguments.schema_workspace.absolute() / "schema"
    source_record = {
        "version": version,
        "schemas": {
            name: hashlib.sha256(contents).hexdigest()
            for name, contents in sorted(
                {
                    **{
                        name: (schema_directory / name).read_bytes() for name in schemas
                    },
                    **{path.name: path.read_bytes() for path in asyncapis},
                }.items()
            )
        },
    }
    for package in (python, javascript):
        shutil.copytree(schema_directory, package / "schema")
        for asyncapi in asyncapis:
            shutil.copyfile(asyncapi, package / "schema" / asyncapi.name)
        shutil.copyfile(templates / "README.md", package / "README.md")
        for license_file in arguments.license_file:
            shutil.copyfile(license_file, package / license_file.name)
        (package / "SOURCE.json").write_text(json.dumps(source_record, indent=2) + "\n")

    python_sources = python / "src"
    python_sources.mkdir()
    # A virtual include path controls Python module names without changing the
    # protobuf package, field names, source file, or generated code.
    compile_proto(
        [
            f"-Irime_api={schema_directory / 'rime'}",
            f"-I{schema_directory}",
            f"--python_out={python_sources}",
            f"--pyi_out={python_sources}",
            *[name.replace("rime/", "rime_api/", 1) for name in schemas],
        ]
    )
    (python_sources / "rime_api/__init__.py").write_text(
        '"""Generated Rime protocol definitions."""\n'
    )
    (python_sources / "rime_api/py.typed").touch()
    (python / "pyproject.toml").write_text(
        (templates / "python.toml").read_text().replace("@VERSION@", version)
    )

    include_paths = [f"-I{schema_directory}"]
    for directory, style in (("esm", "module"), ("commonjs", "legacy_commonjs")):
        generated = javascript / directory
        generated.mkdir()
        compile_proto(
            [
                *include_paths,
                f"--plugin=protoc-gen-es={plugin}",
                f"--es_out={generated}",
                f"--es_opt=target=js+dts,import_extension=js,js_import_style={style}",
                # Google well-known types come from @bufbuild/protobuf. Other
                # imported definitions need generated files inside this package.
                *[
                    path.relative_to(schema_directory).as_posix()
                    for path in sorted(schema_directory.rglob("*.proto"))
                    if not path.relative_to(schema_directory)
                    .as_posix()
                    .startswith("google/protobuf/")
                ],
            ]
        )
        modules = [name.removesuffix(".proto") + "_pb.js" for name in schemas]
        exports = "".join(f"export * from './{module}';\n" for module in modules)
        (generated / "index.d.ts").write_text(exports)
        (generated / "index.js").write_text(
            exports
            if style == "module"
            else "".join(
                f"Object.assign(exports, require('./{module}'));\n"
                for module in modules
            )
        )
    (javascript / "commonjs/package.json").write_text('{"type":"commonjs"}\n')
    (javascript / "package.json").write_text(
        (templates / "javascript.json").read_text().replace("@VERSION@", version)
    )

    previous_directory = Path.cwd()
    try:
        os.chdir(python)
        build_wheel(str(distributions))
        build_sdist(str(distributions))
    finally:
        os.chdir(previous_directory)
    archive_npm(javascript, distributions / f"rime-api-{version}.tgz")
    (distributions / "SOURCE.json").write_text(
        json.dumps(source_record, indent=2) + "\n"
    )


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--schema", action="append", required=True)
    parser.add_argument("--schema-workspace", type=Path, required=True)
    parser.add_argument("--plugin", type=Path, required=True)
    parser.add_argument("--templates", type=Path, required=True)
    parser.add_argument("--version", type=Path, required=True)
    parser.add_argument("--asyncapi", type=Path, action="append", required=True)
    parser.add_argument("--license-file", type=Path, action="append", default=[])
    parser.add_argument("--output", type=Path, required=True)
    build(parser.parse_args())
