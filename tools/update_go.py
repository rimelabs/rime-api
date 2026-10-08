"""Update or verify the checked-in Go protocol files."""

import argparse
import os
from pathlib import Path
import shutil


def update(generated, destination, check=False):
    expected = {
        path.relative_to(generated): path.read_bytes()
        for path in generated.rglob("*")
        if path.is_file()
    }
    managed = {
        path.relative_to(destination)
        for pattern in (
            "*.pb.go",
            "schema/**/*",
            "testdata/fixtures.json",
            "SOURCE.json",
            "LICENSE",
            "NOTICE",
            "PROTOBUF_LICENSE",
        )
        for path in destination.glob(pattern)
        if path.is_file()
    }
    changed = [
        str(name)
        for name in sorted(set(expected) | managed)
        if name not in expected
        or not (destination / name).is_file()
        or (destination / name).read_bytes() != expected[name]
    ]
    if check:
        if changed:
            raise ValueError(
                "Stale Go protocol files; run bazel run //:update_go: "
                + ", ".join(changed)
            )
        return
    for name in managed - set(expected):
        (destination / name).unlink()
    for name in expected:
        (destination / name).parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(generated / name, destination / name)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("generated", type=Path)
    parser.add_argument("--check", action="store_true")
    arguments = parser.parse_args()
    root = Path(os.environ["BUILD_WORKSPACE_DIRECTORY"])
    update(arguments.generated, root / "go", arguments.check)


if __name__ == "__main__":
    main()
