"""Validate schemas and report client compatibility changes without rejecting them."""

import argparse
import json
from pathlib import Path
import re
import shutil
import subprocess
import tempfile


PUBLIC_SCHEMA = "schema/rime/text_to_speech.proto"


def compare(buf, current, baseline):
    current = current.resolve()
    baseline = baseline.resolve()
    # Compile both inputs first. A parse/import failure is never an advisory.
    for workspace in (current, baseline):
        subprocess.run([str(buf), "build", str(workspace)], check=True)
    result = subprocess.run(
        [
            str(buf),
            "breaking",
            str(current),
            "--against",
            str(baseline),
            "--error-format=json",
        ],
        cwd=current,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if result.returncode == 0:
        return []
    if result.returncode != 100:
        raise RuntimeError(result.stderr or result.stdout)
    # Buf also uses 100 for other check errors. Only structured breaking
    # diagnostics from successfully compiled schemas become release advice.
    diagnostics = [json.loads(line) for line in result.stdout.splitlines() if line]
    if not diagnostics or any(
        not isinstance(item, dict) or not item.get("type") or not item.get("message")
        for item in diagnostics
    ):
        raise RuntimeError(result.stderr or "Missing breaking-change diagnostics")
    return diagnostics


def check(buf, workspace, repository, base_revision, report):
    subprocess.run(
        [str(buf), "lint", str(workspace), "--path", PUBLIC_SCHEMA],
        cwd=workspace,
        check=True,
    )
    references = [base_revision] if base_revision else []
    tags = subprocess.check_output(
        ["git", "tag", "--merged", "HEAD", "--list", "v*", "--sort=-version:refname"],
        cwd=repository,
        text=True,
    ).splitlines()
    current_revision = subprocess.check_output(
        ["git", "rev-parse", "HEAD"],
        cwd=repository,
        text=True,
    ).strip()
    for tag in tags:
        revision = subprocess.check_output(
            ["git", "rev-list", "-n", "1", tag],
            cwd=repository,
            text=True,
        ).strip()
        if re.fullmatch(r"v\d+\.\d+\.\d+", tag) and revision != current_revision:
            references.append(tag)
            break
    sections = [
        "# Public API compatibility\n",
        "The schemas follow rimelabs/rime. Breaking changes require release review; they do not stop schema sync.\n",
    ]
    for reference in dict.fromkeys(references):
        with tempfile.TemporaryDirectory() as temporary:
            baseline = Path(temporary) / "baseline"
            shutil.copytree(workspace, baseline, copy_function=shutil.copyfile)
            previous = subprocess.check_output(
                ["git", "show", f"{reference}:{PUBLIC_SCHEMA}"],
                cwd=repository,
            )
            (baseline / PUBLIC_SCHEMA).write_bytes(previous)
            diagnostics = compare(buf, workspace, baseline)
            sections.append(f"## Compared with {reference}\n")
            sections.extend(
                f"- `{item['type']}`: {item['message']}\n" for item in diagnostics
            )
            if diagnostics:
                print(
                    "::warning::Public API breaking changes detected. Review the compatibility report before release."
                )
            else:
                sections.append("No breaking Protobuf changes detected.\n")
    if not references:
        sections.append("No previous release or PR base is available.\n")
    report.write_text("\n".join(sections))
    print(report.read_text())


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--buf", type=Path, required=True)
    parser.add_argument("--workspace", type=Path, required=True)
    parser.add_argument("--repository", type=Path, required=True)
    parser.add_argument("--base-revision", default="")
    parser.add_argument("--report", type=Path, required=True)
    arguments = parser.parse_args()
    check(
        arguments.buf.resolve(),
        arguments.workspace.resolve(),
        arguments.repository.resolve(),
        arguments.base_revision,
        arguments.report.resolve(),
    )
