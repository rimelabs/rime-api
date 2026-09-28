"""Exercise release selection and finalization without changing external services."""

import base64
import copy
import hashlib
import io
import json
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch
import zipfile

from tools import release


class ReleaseAutomationTest(unittest.TestCase):
    def setUp(self):
        self.commit = "a" * 40
        self.version = "0.2.0"
        self.run = {
            "id": 123,
            "repository": {"full_name": release.REPOSITORY},
            "head_repository": {"full_name": release.REPOSITORY},
            "head_branch": "main",
            "event": "push",
            "status": "completed",
            "conclusion": "success",
            "path": ".github/workflows/check.yaml",
            "head_sha": self.commit,
        }
        self.pull_request = {
            "number": 42,
            "merged_at": "2026-09-28T12:00:00Z",
            "merge_commit_sha": self.commit,
            "base": {"ref": "main", "repo": {"full_name": release.REPOSITORY}},
            "head": {
                "ref": "release-please--branches--main",
                "repo": {"full_name": release.REPOSITORY},
            },
            "user": {"login": "github-actions[bot]"},
            "labels": [{"name": "autorelease: pending"}],
        }
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.directory = Path(temporary.name)
        self.files = {
            name: name.encode()
            for names in release.archive_names(self.version).values()
            for name in names
        }
        self.files["SOURCE.json"] = b"{}"
        self.record = {
            "commit": self.commit,
            "version": self.version,
            "run_id": 123,
            "sha256": {
                name: hashlib.sha256(data).hexdigest()
                for name, data in self.files.items()
            },
        }
        for name, data in self.files.items():
            (self.directory / name).write_bytes(data)
        (self.directory / "RELEASE.json").write_text(json.dumps(self.record))

    def commit_file(self, commit, path):
        if path == ".release-please-manifest.json":
            return json.dumps({".": self.version})
        return "0.1.0" if commit.endswith("^") else self.version

    def select(self, run=None, pull_request=None):
        with (
            patch.object(release, "github", return_value=self.run),
            patch.object(
                release,
                "github_pages",
                return_value=[pull_request or self.pull_request],
            ),
            patch.object(release, "commit_file", side_effect=self.commit_file),
        ):
            return release.select_release({"workflow_run": run or self.run})

    def test_selects_exact_merged_release_commit(self):
        self.assertEqual(
            self.select(), {"run_id": 123, "version": self.version, "registry": "both"}
        )

    def test_ignores_failed_branch_and_fork_checks(self):
        for field, value in [
            ("conclusion", "failure"),
            ("conclusion", "cancelled"),
            ("head_branch", "release-please--branches--main"),
            ("event", "pull_request"),
            ("event", "schedule"),
            ("head_repository", {"full_name": "fork/rime-api"}),
        ]:
            with self.subTest(field=field, value=value):
                self.assertEqual(self.select(dict(self.run, **{field: value})), {})

    def test_ignores_ordinary_unmerged_or_different_commit_pr(self):
        for field, value in [
            ("merged_at", None),
            ("merge_commit_sha", "b" * 40),
            ("user", {"login": "someone"}),
            ("labels", []),
            ("head", {"ref": "feature", "repo": {"full_name": release.REPOSITORY}}),
            ("head", {"ref": "release-please--branches--main", "repo": None}),
        ]:
            with self.subTest(field=field):
                request = dict(self.pull_request, **{field: value})
                self.assertEqual(self.select(pull_request=request), {})

    def test_rejects_changed_run_or_manifest(self):
        with self.assertRaisesRegex(ValueError, "Completed commit differs"):
            self.select(dict(self.run, head_sha="b" * 40))
        with (
            patch.object(release, "github", return_value=self.run),
            patch.object(release, "github_pages", return_value=[self.pull_request]),
            patch.object(
                release, "commit_file", side_effect=[self.version, '{".": "9.0.0"}']
            ),
            self.assertRaisesRegex(ValueError, "manifest"),
        ):
            release.select_release({"workflow_run": self.run})

    def test_version_must_change_to_release(self):
        with (
            patch.object(release, "github", return_value=self.run),
            patch.object(release, "github_pages", return_value=[self.pull_request]),
            patch.object(
                release,
                "commit_file",
                side_effect=[
                    self.version,
                    json.dumps({".": self.version}),
                    self.version,
                ],
            ),
            self.assertRaisesRegex(ValueError, "did not change"),
        ):
            release.select_release({"workflow_run": self.run})

    def test_manual_recovery_preserves_original_run_and_registry(self):
        inputs = {"run_id": "100", "version": "0.0.1", "registry": "pypi"}
        self.assertEqual(
            release.select_release({"inputs": inputs}), dict(inputs, run_id=100)
        )
        with self.assertRaises(ValueError):
            release.select_release(
                {"inputs": dict(inputs, version="0.0.1\nregistry=npm")}
            )

    def prepare_with_record(self, original_run_id, published=False, expired=False):
        archive = io.BytesIO()
        with zipfile.ZipFile(archive, "w") as output:
            for name, data in self.files.items():
                output.writestr(name, data)
        source_archive = archive.getvalue()
        original_record = dict(
            self.record,
            run_id=original_run_id,
            artifact_id=44,
            artifact_digest="sha256:" + hashlib.sha256(source_archive).hexdigest(),
        )
        record_archive = io.BytesIO()
        with zipfile.ZipFile(record_archive, "w") as output:
            output.writestr("RELEASE.json", json.dumps(original_record))
        record_data = record_archive.getvalue()
        source_artifact = {
            "id": 44,
            "name": "rime-api-dist",
            "expired": False,
            "workflow_run": {"head_sha": self.commit},
            "digest": original_record["artifact_digest"],
        }
        claim_artifact = {
            "id": 55,
            "name": f"release-dist-{self.version}",
            "expired": expired,
            "workflow_run": {"id": 987},
            "digest": "sha256:" + hashlib.sha256(record_data).hexdigest(),
        }

        def github(path):
            if path == "actions/runs/123":
                return dict(self.run, run_attempt=1)
            self.assertEqual(path, "actions/runs/987")
            return dict(
                self.run,
                id=987,
                event="workflow_dispatch",
                path=".github/workflows/release.yaml",
            )

        def github_pages(path, key=None):
            if path.endswith("/jobs?per_page=100"):
                return [
                    {"name": name, "conclusion": "success"}
                    for name in ["build", "python (3.13)", "javascript (24)"]
                ]
            if path == "actions/runs/123/artifacts?per_page=100":
                return [source_artifact]
            if path == "releases?per_page=100":
                return (
                    [
                        {
                            "tag_name": f"v{self.version}",
                            "assets": [{"name": "RELEASE.json", "id": 66}],
                        }
                    ]
                    if published
                    else []
                )
            self.assertEqual(
                path, f"actions/artifacts?name=release-dist-{self.version}&per_page=100"
            )
            return [claim_artifact]

        def command(*arguments):
            if arguments[:2] == ("git", "merge-base"):
                return b""
            if arguments[:2] == ("git", "show"):
                return (
                    self.version.encode()
                    if arguments[2].endswith(":VERSION")
                    else b"schema"
                )
            self.assertEqual(arguments[:2], ("gh", "api"))
            path = arguments[2]
            if path.endswith("/artifacts/44/zip"):
                return source_archive
            if path.endswith("/artifacts/55/zip"):
                return record_data
            self.assertTrue(path.endswith("/releases/assets/66"))
            return json.dumps(original_record).encode()

        destination = self.directory / "prepared"
        with (
            patch.object(release, "github", side_effect=github),
            patch.object(release, "github_pages", side_effect=github_pages),
            patch.object(release, "command", side_effect=command),
            patch.object(release, "validate_archives"),
        ):
            release.prepare(123, self.version, destination)
        return json.loads((destination / "RELEASE.json").read_text())

    def test_partial_publication_reserves_the_original_check_run(self):
        with self.assertRaisesRegex(ValueError, "already assigned to check run 100"):
            self.prepare_with_record(100)

    def test_completed_release_rejects_another_check_run_before_publication(self):
        with self.assertRaisesRegex(ValueError, "already assigned to check run 100"):
            self.prepare_with_record(100, published=True)

    def test_same_check_run_can_resume_partial_publication(self):
        self.assertEqual(self.prepare_with_record(123)["run_id"], 123)

    def test_expired_reservation_cannot_be_silently_replaced(self):
        with self.assertRaisesRegex(ValueError, "expired"):
            self.prepare_with_record(100, expired=True)

    def test_incomplete_registry_cannot_finalize(self):
        with (
            patch.object(release, "registry_metadata", return_value=None),
            patch.object(release, "command") as command,
        ):
            self.assertFalse(release.published_release(self.directory))
            with self.assertRaisesRegex(ValueError, "Both registries"):
                release.finalize_release(self.directory)
            command.assert_not_called()

    def test_altered_local_archive_cannot_finalize(self):
        (self.directory / next(iter(self.files))).write_bytes(b"changed")
        with self.assertRaisesRegex(ValueError, "checksum"):
            release.published_release(self.directory)

    def test_readiness_requires_complete_matching_files_on_both_registries(self):
        names = release.archive_names(self.version)
        metadata = {
            "npm": {
                "name": "@rimelabs/api",
                "version": self.version,
                "dist": {
                    "integrity": "sha512-"
                    + base64.b64encode(
                        hashlib.sha512(self.files[names["npm"][0]]).digest()
                    ).decode(),
                    "tarball": "https://registry.npmjs.org/package.tgz",
                },
            },
            "pypi": {
                "info": {"version": self.version},
                "urls": [
                    {
                        "filename": name,
                        "digests": {"sha256": self.record["sha256"][name]},
                        "url": f"https://files.pythonhosted.org/{name}",
                    }
                    for name in names["pypi"]
                ],
            },
        }
        with patch.object(
            release,
            "registry_metadata",
            side_effect=lambda registry, version: metadata[registry],
        ):
            self.assertTrue(release.published_release(self.directory))
            metadata["pypi"]["urls"].pop()
            self.assertFalse(release.published_release(self.directory))
            metadata["npm"]["dist"]["integrity"] = "sha512-different"
            with self.assertRaisesRegex(ValueError, "different bytes"):
                release.published_release(self.directory)

    def finalize(self, tag_target=None, existing=None, asset_contents=None):
        commands = []

        def command(*arguments):
            commands.append(arguments)
            if arguments[:2] == ("git", "ls-remote"):
                if tag_target:
                    # Annotated tag object differs from the commit it resolves to.
                    return f"{'c' * 40}\trefs/tags/v{self.version}\n{tag_target}\trefs/tags/v{self.version}^{{}}\n".encode()
                return b""
            if arguments[:2] == ("gh", "api") and "assets/" in arguments[2]:
                return asset_contents
            return b""

        draft = {"tag_name": f"v{self.version}", "draft": True, "assets": []}

        def list_releases(path):
            self.assertEqual(path, "releases?per_page=100")
            if existing:
                return [existing]
            if any(item[:3] == ("gh", "release", "create") for item in commands):
                return [draft]
            return []

        with (
            patch.object(release, "published_release", return_value=True),
            patch.object(release, "commit_file", return_value=self.version),
            patch.object(release, "release_pull_request", return_value=42),
            patch.object(release, "github_pages", side_effect=list_releases),
            patch.object(
                release,
                "github",
                side_effect=AssertionError(
                    "Drafts cannot be fetched through releases/tags"
                ),
            ),
            patch.object(release, "command", side_effect=command),
            patch.object(
                release.subprocess,
                "run",
                return_value=subprocess.CompletedProcess(
                    [],
                    0,
                    b"# Changelog\n\n## 0.2.0\n\n- Public changes.\n\n## 0.1.0\n\n- Old changes.\n",
                ),
            ),
        ):
            release.finalize_release(self.directory)
        return commands

    def test_creates_tag_at_tested_commit_and_publishes_after_assets(self):
        commands = self.finalize()
        create_tag = next(
            item for item in commands if f"repos/{release.REPOSITORY}/git/refs" in item
        )
        self.assertIn(f"sha={self.commit}", create_tag)
        uploads = [
            index
            for index, item in enumerate(commands)
            if item[:3] == ("gh", "release", "upload")
        ]
        publication = next(
            index for index, item in enumerate(commands) if "--draft=false" in item
        )
        self.assertEqual(len(uploads), len(self.files) + 1)
        self.assertGreater(publication, max(uploads))
        notes = (self.directory / "release-notes.md").read_text()
        self.assertIn("Public changes", notes)
        self.assertNotIn("Old changes", notes)
        self.assertIn("autorelease: tagged", commands[-1])

    def test_rejects_existing_tag_on_another_commit(self):
        with self.assertRaisesRegex(ValueError, "another commit"):
            self.finalize(tag_target="b" * 40)

    def test_resumes_existing_draft_without_replacing_matching_asset(self):
        name = next(iter(self.files))
        existing = {
            "tag_name": f"v{self.version}",
            "draft": True,
            "assets": [{"name": name, "id": 99}],
        }
        commands = self.finalize(self.commit, existing, self.files[name])
        self.assertFalse(
            any(item[:3] == ("gh", "release", "create") for item in commands)
        )
        self.assertFalse(
            any(
                item[:3] == ("gh", "release", "upload")
                and str(self.directory / name) in item
                for item in commands
            )
        )
        with self.assertRaisesRegex(ValueError, "asset differs"):
            self.finalize(self.commit, existing, b"different")
        complete = copy.deepcopy(existing)
        complete["draft"] = False
        commands = self.finalize(self.commit, complete, self.files[name])
        self.assertFalse(any("--draft=false" in item for item in commands))


if __name__ == "__main__":
    unittest.main()
