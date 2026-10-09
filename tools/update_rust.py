"""Copy Bazel-generated Rust files and release metadata into the Cargo package."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import re


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("generated", type=Path)
    parser.add_argument("--check", action="store_true")
    arguments = parser.parse_args()
    root = Path(os.environ["BUILD_WORKSPACE_DIRECTORY"])
    package = root / "rust"
    version = (root / "VERSION").read_text().strip()
    expected = {
        package / "src/generated" / path.name: path.read_bytes()
        for path in arguments.generated.glob("*.rs")
    }
    if {path.name for path in expected} != {
        "rime.rs",
        "rime.serde.rs",
        "google.rpc.rs",
        "google.rpc.serde.rs",
    }:
        raise SystemExit("Rust generator did not produce the complete protocol")
    for name in ("LICENSE", "NOTICE"):
        expected[package / name] = (root / name).read_bytes()
    schemas = [
        "rime/text_to_speech.proto",
        "rime/speech_to_text.proto",
        "text_to_speech.asyncapi.yaml",
        "speech_to_text.asyncapi.yaml",
    ]
    source = {
        "version": version,
        "schemas": {
            name: hashlib.sha256((root / "schema" / name).read_bytes()).hexdigest()
            for name in sorted(schemas)
        },
    }
    expected[package / "SOURCE.json"] = (json.dumps(source, indent=2) + "\n").encode()
    manifest = package / "Cargo.toml"
    expected[manifest] = re.sub(
        r'^version = "[^"]+"',
        f'version = "{version}"',
        manifest.read_text(),
        count=1,
        flags=re.M,
    ).encode()
    stale = set((package / "src/generated").glob("*.rs")) - set(expected)
    changed = [
        path
        for path, contents in expected.items()
        if not path.exists() or path.read_bytes() != contents
    ]
    if arguments.check:
        if changed or stale:
            raise SystemExit("Rust sources are stale. Run bazel run //:update_rust")
        return
    for path in stale:
        path.unlink()
    for path in changed:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(expected[path])


if __name__ == "__main__":
    main()
