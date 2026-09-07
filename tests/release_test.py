"""Check release source validation and recovery from partial publication."""

import base64
import copy
import hashlib
import json
from pathlib import Path
import sys
import unittest
from unittest.mock import patch

from tools import release


PACKAGES = Path(sys.argv.pop(1))


class ReleaseTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.files = {
            path.name: path.read_bytes() for path in (PACKAGES / "dist").iterdir()
        }
        source = json.loads(cls.files["SOURCE.json"])
        cls.version = source["version"]
        cls.schemas = {
            name: (PACKAGES / "javascript/schema" / name).read_bytes()
            for name in [source["schema"], source["asyncapi"]["schema"]]
        }

    def test_accepts_built_archives(self):
        release.validate_archives(self.files, self.version, self.schemas)

    def test_rejects_wrong_source_commit(self):
        for name in self.schemas:
            with self.subTest(schema=name):
                schemas = dict(self.schemas, **{name: self.schemas[name] + b"\n"})
                with self.assertRaisesRegex(ValueError, "selected commit"):
                    release.validate_archives(self.files, self.version, schemas)

    def test_rejects_extra_artifact_files(self):
        with self.assertRaisesRegex(ValueError, "Unexpected artifact files"):
            release.validate_archives(
                dict(self.files, unexpected=b""), self.version, self.schemas
            )

    def test_rejects_mismatched_archive_version(self):
        files = dict(self.files)
        names = release.archive_names(self.version)
        replacement = release.archive_names("99.0.0")
        for registry in names:
            for original, renamed in zip(names[registry], replacement[registry]):
                files[renamed] = files.pop(original)
        source = json.loads(files["SOURCE.json"])
        source["version"] = "99.0.0"
        files["SOURCE.json"] = json.dumps(source).encode()
        with self.assertRaisesRegex(ValueError, "npm metadata differs"):
            release.validate_archives(files, "99.0.0", self.schemas)

    def test_requires_successful_main_run(self):
        run = {
            "id": 123,
            "repository": {"full_name": release.REPOSITORY},
            "head_repository": {"full_name": release.REPOSITORY},
            "head_branch": "main",
            "event": "push",
            "status": "completed",
            "conclusion": "success",
            "path": ".github/workflows/check.yaml",
            "head_sha": "a" * 40,
        }
        release.validate_run(run, 123)
        for key, value in {
            "id": 124,
            "head_repository": {"full_name": "fork/rime-api"},
            "head_branch": "feature",
            "event": "pull_request",
            "status": "in_progress",
            "conclusion": "failure",
            "path": ".github/workflows/release.yaml",
            "head_sha": "invalid",
        }.items():
            with self.subTest(field=key), self.assertRaises(ValueError):
                release.validate_run(dict(run, **{key: value}), 123)

    def test_npm_retry_requires_identical_archive(self):
        name = release.archive_names(self.version)["npm"][0]
        metadata = {
            "name": "@rimelabs/api",
            "version": self.version,
            "dist": {
                "integrity": "sha512-"
                + base64.b64encode(hashlib.sha512(self.files[name]).digest()).decode(),
                "tarball": "https://registry.npmjs.org/archive.tgz",
            },
        }
        self.assertEqual(
            release.compare_registry("npm", metadata, self.files, self.version),
            {name: metadata["dist"]["tarball"]},
        )
        metadata["dist"]["integrity"] = "sha512-different"
        with self.assertRaisesRegex(ValueError, "different bytes"):
            release.compare_registry("npm", metadata, self.files, self.version)

    def test_partial_pypi_release_accepts_only_matching_files(self):
        name = release.archive_names(self.version)["pypi"][0]
        metadata = {
            "info": {"version": self.version},
            "urls": [
                {
                    "filename": name,
                    "digests": {"sha256": hashlib.sha256(self.files[name]).hexdigest()},
                    "url": "https://files.pythonhosted.org/archive.whl",
                }
            ],
        }
        self.assertEqual(
            len(release.compare_registry("pypi", metadata, self.files, self.version)), 1
        )
        for field, value in [
            ("filename", "unexpected.whl"),
            ("digests", {"sha256": "different"}),
        ]:
            invalid = copy.deepcopy(metadata)
            invalid["urls"][0][field] = value
            with self.subTest(field=field), self.assertRaises(ValueError):
                release.compare_registry("pypi", invalid, self.files, self.version)

    def test_missing_registry_version_is_publishable(self):
        for registry in ["npm", "pypi"]:
            self.assertEqual(
                release.compare_registry(registry, None, self.files, self.version), {}
            )

    def test_registry_errors_do_not_mean_package_is_missing(self):
        for status in [401, 403, 429, 500]:
            error = release.HTTPError("https://example.com", status, "test", {}, None)
            with (
                patch.object(release, "fetch", side_effect=error),
                self.assertRaises(release.HTTPError),
            ):
                release.registry_metadata("npm", self.version)


if __name__ == "__main__":
    unittest.main()
