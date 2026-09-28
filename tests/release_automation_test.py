"""Exercise release selection and finalization without changing external services."""

import base64
import copy
import hashlib
import json
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch

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
        with (
            patch.object(release, "published_release", return_value=True),
            patch.object(release, "commit_file", return_value=self.version),
            patch.object(release, "release_pull_request", return_value=42),
            patch.object(
                release, "github_pages", return_value=[existing] if existing else []
            ),
            patch.object(release, "github", return_value=draft),
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
