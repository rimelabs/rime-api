"""Check the release tag and package metadata."""

import json
from pathlib import Path
import re
import sys
import tomllib

repository = Path(__file__).resolve().parents[1]
version = (repository / "VERSION").read_text().strip()
if not re.fullmatch(r"(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)", version):
    raise SystemExit("VERSION must contain a stable major.minor.patch version")
if sys.argv[1:] != [f"v{version}"]:
    raise SystemExit(f"Release tag must be v{version}")
python = tomllib.loads((repository / "packages/python.toml").read_text())
javascript = json.loads((repository / "packages/javascript.json").read_text())
for metadata, name in [(python["project"], "rime-api"), (javascript, "@rimelabs/api")]:
    if metadata["name"] != name or metadata["version"] != "@VERSION@":
        raise SystemExit(f"Unexpected package name or version template for {name}")
    if metadata["license"] != "Apache-2.0":
        raise SystemExit(f"Unexpected license for {name}")
if "Apache License" not in (repository / "LICENSE").read_text():
    raise SystemExit("The Apache license text is missing")
print(f"Release metadata is ready for v{version}")
