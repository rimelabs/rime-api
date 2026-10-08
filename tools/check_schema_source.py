"""Verify exported schemas against a source revision without executing PR code."""

import argparse
import ast
import base64
import json
from functools import cache
from pathlib import Path
import re
import subprocess


DESTINATION = "rimelabs/rime-api"
SOURCE = "rimelabs/rime"
REVISION_FILE = "sync/SOURCE_REVISION"


def export_paths(configuration):
    """Read literal Copybara file lists, without evaluating Starlark."""
    tree = ast.parse(configuration)
    workflows = [
        node
        for node in ast.walk(tree)
        if isinstance(node, ast.Call)
        and isinstance(node.func, ast.Attribute)
        and isinstance(node.func.value, ast.Name)
        and node.func.value.id == "core"
        and node.func.attr == "workflow"
    ]
    if len(workflows) != 1:
        raise ValueError("Expected one Copybara workflow")
    keywords = {keyword.arg: keyword.value for keyword in workflows[0].keywords}
    paths = {}
    for name in ("origin_files", "destination_files"):
        call = keywords[name]
        if not (
            isinstance(call, ast.Call)
            and isinstance(call.func, ast.Name)
            and call.func.id == "glob"
            and len(call.args) == 1
            and not call.keywords
        ):
            raise ValueError("Export paths must be literal glob lists")
        values = ast.literal_eval(call.args[0])
        if not isinstance(values, list) or not values:
            raise ValueError("Export paths must be a nonempty list")
        prefix = "interfaces/" if name == "origin_files" else "schema/"
        if any(
            not isinstance(value, str)
            or not re.fullmatch(
                re.escape(prefix) + r"[a-zA-Z0-9_/-]+\.(?:proto|asyncapi\.yaml)", value
            )
            or ".." in value
            for value in values
        ):
            raise ValueError("Export paths must name individual API schema files")
        if len(values) != len(set(values)):
            raise ValueError("Duplicate export path")
        paths[name] = values
    expected = [
        value.replace("interfaces/", "schema/", 1) for value in paths["origin_files"]
    ]
    if sorted(expected) != sorted(paths["destination_files"]):
        raise ValueError("Source and destination export lists do not match")
    # Changes outside the two lists must receive a separate trusted workflow review.
    for keyword in workflows[0].keywords:
        if keyword.arg in paths:
            keyword.value = ast.Constant(value="export-list")
    return dict(zip(paths["origin_files"], expected)), ast.dump(tree)


def verify(
    candidate_revision,
    trusted_configuration,
    read_file,
    source_is_on_main,
    allow_export_change=False,
):
    if not re.fullmatch(r"[0-9a-f]{40}", candidate_revision):
        raise ValueError("Candidate must be a full commit ID")
    configuration = read_file(DESTINATION, candidate_revision, "copy.bara.sky").decode()
    paths, structure = export_paths(configuration)
    trusted_paths, trusted_structure = export_paths(trusted_configuration)
    if structure != trusted_structure:
        raise ValueError(
            "Copybara workflow changes require a separate trusted policy review"
        )
    if paths != trusted_paths and not allow_export_change:
        raise ValueError(
            "Export list changed; a maintainer must review and dispatch the migration check"
        )
    source_revision = (
        read_file(DESTINATION, candidate_revision, REVISION_FILE).decode().strip()
    )
    if not re.fullmatch(r"[0-9a-f]{40}", source_revision):
        raise ValueError("Source revision must be a full commit ID")
    if not source_is_on_main(source_revision):
        raise ValueError("Source revision is not an ancestor of rime/main")
    mismatches = []
    for source_path, destination_path in paths.items():
        actual = read_file(DESTINATION, candidate_revision, destination_path)
        expected = read_file(SOURCE, source_revision, source_path)
        if actual != expected:
            mismatches.append(destination_path)
    if mismatches:
        # Do not print private source content, hashes, or API responses into Actions logs.
        raise ValueError(
            "Schemas differ from their recorded source: " + ", ".join(mismatches)
        )
    return len(paths)


def api(path):
    response = subprocess.run(["gh", "api", path], capture_output=True, check=False)
    if response.returncode:
        raise ValueError(
            "GitHub source verification request failed; check access and revision"
        )
    return json.loads(response.stdout)


@cache
def candidate_tree(revision):
    result = api(f"repos/{DESTINATION}/git/trees/{revision}?recursive=1")
    if result.get("truncated"):
        raise ValueError("Candidate tree is too large to verify")
    return {entry["path"]: entry for entry in result["tree"]}


def read_file(repository, revision, path):
    if repository == DESTINATION:
        entry = candidate_tree(revision).get(path, {})
        if entry.get("mode") != "100644" or entry.get("type") != "blob":
            raise ValueError(
                "Schema and provenance files must be regular non-executable files"
            )
    result = api(f"repos/{repository}/contents/{path}?ref={revision}")
    if result.get("type") != "file" or result.get("encoding") != "base64":
        raise ValueError("Expected a regular schema or metadata file")
    return base64.b64decode(result["content"], validate=False)


def source_is_on_main(revision):
    result = api(f"repos/{SOURCE}/compare/{revision}...main")
    return result["status"] in ("ahead", "identical")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--candidate", required=True)
    parser.add_argument(
        "--trusted-configuration", type=Path, default=Path("copy.bara.sky")
    )
    parser.add_argument("--allow-export-change", action="store_true")
    arguments = parser.parse_args()
    try:
        count = verify(
            arguments.candidate,
            arguments.trusted_configuration.read_text(),
            read_file,
            source_is_on_main,
            arguments.allow_export_change,
        )
    except (ValueError, KeyError, SyntaxError, UnicodeError) as error:
        # Parse errors can contain candidate text. Keep the public failure message bounded.
        message = (
            str(error)
            if type(error) is ValueError
            else "Invalid schema verification metadata"
        )
        print(message)
        return 1
    print(f"Verified {count} schemas against their recorded source revision.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
