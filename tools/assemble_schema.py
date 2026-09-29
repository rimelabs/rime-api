"""Stage the public schema and its Bazel dependencies for Buf and generators."""

import argparse
from pathlib import Path
import shutil


def assemble(output, configuration, sources, roots):
    output.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(configuration, output / "buf.yaml")
    # Match virtual import roots before a less specific workspace root.
    roots = sorted(roots, key=lambda root: len(root.parts), reverse=True)
    for source in sources:
        for root in roots:
            if source.is_relative_to(root):
                destination = output / "schema" / source.relative_to(root)
                destination.parent.mkdir(parents=True, exist_ok=True)
                if (
                    destination.exists()
                    and destination.read_bytes() != source.read_bytes()
                ):
                    raise ValueError(f"Conflicting Protobuf import: {destination}")
                shutil.copyfile(source, destination)
                break
        else:
            raise ValueError(f"No Protobuf import root for {source}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--configuration", type=Path, required=True)
    parser.add_argument("--proto-path", type=Path, action="append", default=[])
    parser.add_argument("--source", type=Path, action="append", default=[])
    arguments = parser.parse_args()
    assemble(
        arguments.output,
        arguments.configuration,
        arguments.source,
        arguments.proto_path,
    )
