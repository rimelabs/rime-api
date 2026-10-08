"""Generate the public Go protocol from the canonical schema workspace."""

import argparse
import hashlib
import json
from pathlib import Path
import shutil
from grpc_tools import protoc

MODULE = "github.com/rimelabs/rime-api/go"


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--workspace", type=Path, required=True)
    parser.add_argument("--fixtures", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--go-plugin", type=Path, required=True)
    parser.add_argument("--grpc-plugin", type=Path, required=True)
    parser.add_argument("--license", type=Path, action="append", default=[])
    parser.add_argument("--asyncapi", type=Path, action="append", default=[])
    args = parser.parse_args()
    schema = args.workspace / "schema"
    args.output.mkdir(parents=True, exist_ok=True)
    schemas = sorted(
        path.relative_to(schema).as_posix()
        for path in (schema / "rime").glob("*.proto")
    )
    mappings = [f"M{name}={MODULE};rimeapi" for name in schemas]
    arguments = [
        f"-I{schema}",
        f"--plugin=protoc-gen-go={args.go_plugin.absolute()}",
        f"--plugin=protoc-gen-go-grpc={args.grpc_plugin.absolute()}",
        f"--go_out={args.output}",
        f"--go_opt=module={MODULE}",
        f"--go-grpc_out={args.output}",
        f"--go-grpc_opt=module={MODULE}",
        *[f"--go_opt={value}" for value in mappings],
        *[f"--go-grpc_opt={value}" for value in mappings],
        *schemas,
    ]
    if protoc.main(["protoc", *arguments]):
        raise RuntimeError("Go protocol generation failed")
    shutil.copytree(schema, args.output / "schema")
    for path in args.asyncapi:
        shutil.copyfile(path, args.output / "schema" / path.name)
    for path in args.license:
        shutil.copyfile(path, args.output / path.name)
    (args.output / "testdata").mkdir()
    shutil.copyfile(args.fixtures, args.output / "testdata/fixtures.json")
    record = {
        "schemas": {
            path.relative_to(args.output / "schema").as_posix(): hashlib.sha256(
                path.read_bytes()
            ).hexdigest()
            for path in sorted((args.output / "schema").rglob("*"))
            if path.is_file()
        }
    }
    (args.output / "SOURCE.json").write_text(json.dumps(record, indent=2) + "\n")


if __name__ == "__main__":
    main()
