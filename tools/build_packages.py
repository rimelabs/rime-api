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
    source = arguments.source.absolute()
    proto_root = arguments.proto_root.absolute()
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
    asyncapi = arguments.asyncapi.absolute()
    source_record = {
        "version": version,
        "schema": "rime/text_to_speech.proto",
        "sha256": hashlib.sha256(source.read_bytes()).hexdigest(),
        "asyncapi": {
            "schema": "text_to_speech.asyncapi.yaml",
            "sha256": hashlib.sha256(asyncapi.read_bytes()).hexdigest(),
        },
    }
    for package in (python, javascript):
        (package / "schema/rime").mkdir(parents=True)
        shutil.copyfile(source, package / "schema/rime/text_to_speech.proto")
        shutil.copyfile(asyncapi, package / "schema/text_to_speech.asyncapi.yaml")
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
            f"-Irime_api={source.parent}",
            f"--python_out={python_sources}",
            f"--pyi_out={python_sources}",
            "rime_api/text_to_speech.proto",
        ]
    )
    (python_sources / "rime_api/__init__.py").write_text(
        '"""Generated Rime protocol definitions."""\n'
    )
    (python_sources / "rime_api/py.typed").touch()
    (python / "pyproject.toml").write_text(
        (templates / "python.toml").read_text().replace("@VERSION@", version)
    )

    include_paths = [f"-I{proto_root}"]
    include_paths.extend(f"-I{path.absolute()}" for path in arguments.proto_path)
    for directory, style in (("esm", "module"), ("commonjs", "legacy_commonjs")):
        generated = javascript / directory
        generated.mkdir()
        compile_proto(
            [
                *include_paths,
                f"--plugin=protoc-gen-es={plugin}",
                f"--es_out={generated}",
                f"--es_opt=target=js+dts,import_extension=js,js_import_style={style}",
                "rime/text_to_speech.proto",
            ]
        )
        exports = "export * from './rime/text_to_speech_pb.js';\n"
        (generated / "index.d.ts").write_text(exports)
        (generated / "index.js").write_text(
            exports
            if style == "module"
            else "module.exports = require('./rime/text_to_speech_pb.js');\n"
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
    parser.add_argument("--source", type=Path, required=True)
    parser.add_argument("--proto-root", type=Path, required=True)
    parser.add_argument("--proto-path", type=Path, action="append", default=[])
    parser.add_argument("--plugin", type=Path, required=True)
    parser.add_argument("--templates", type=Path, required=True)
    parser.add_argument("--version", type=Path, required=True)
    parser.add_argument("--asyncapi", type=Path, required=True)
    parser.add_argument("--license-file", type=Path, action="append", default=[])
    parser.add_argument("--output", type=Path, required=True)
    build(parser.parse_args())
