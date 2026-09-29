"""Check publication delays without waiting or changing public registries."""

import base64
import hashlib
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

from tools import release


class RegistryVerificationTest(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.directory = Path(temporary.name)
        self.download = self.directory / "download"
        self.version = "0.1.0"
        self.name = release.archive_names(self.version)["npm"][0]
        self.archive = b"original tested archive"
        (self.directory / self.name).write_bytes(self.archive)
        (self.directory / "RELEASE.json").write_text(
            json.dumps(
                {
                    "version": self.version,
                    "run_id": 123,
                    "sha256": {self.name: hashlib.sha256(self.archive).hexdigest()},
                }
            )
        )
        self.metadata = {
            "name": "@rimelabs/api",
            "version": self.version,
            "dist": {
                "integrity": "sha512-"
                + base64.b64encode(hashlib.sha512(self.archive).digest()).decode(),
                "tarball": "https://registry.npmjs.org/archive.tgz",
            },
        }
        self.elapsed = 0
        self.sleeps = []
        self.addCleanup(patch.stopall)
        patch.object(
            release.time, "monotonic", side_effect=lambda: self.elapsed
        ).start()
        patch.object(release.time, "sleep", side_effect=self.sleep).start()

    def sleep(self, seconds):
        self.sleeps.append(seconds)
        self.elapsed += seconds

    def verify(self):
        release.check_registry("npm", self.directory, self.download)

    def test_package_available_after_three_minutes_is_verified(self):
        with (
            patch.object(
                release,
                "registry_metadata",
                side_effect=lambda *args: (
                    self.metadata if self.elapsed >= 180 else None
                ),
            ),
            patch.object(release, "fetch", return_value=self.archive),
        ):
            self.verify()
        self.assertEqual(self.elapsed, 180)
        self.assertEqual((self.download / self.name).read_bytes(), self.archive)

    def test_still_missing_after_ten_minutes_fails_with_recovery_details(self):
        with (
            patch.object(release, "registry_metadata", return_value=None),
            patch.object(release, "fetch") as fetch,
            self.assertRaisesRegex(ValueError, "npm.*0.1.0.*600.*123"),
        ):
            self.verify()
        self.assertEqual(self.elapsed, 600)
        self.assertFalse(self.download.exists())
        fetch.assert_not_called()

    def test_prepublication_check_does_not_wait(self):
        with patch.object(release, "registry_metadata", return_value=None) as metadata:
            release.check_registry("npm", self.directory, None)
        metadata.assert_called_once()
        self.assertEqual(self.sleeps, [])

    def test_archive_availability_can_lag_metadata(self):
        missing = release.HTTPError(
            self.metadata["dist"]["tarball"], 404, "not ready", {}, None
        )
        with (
            patch.object(release, "registry_metadata", return_value=self.metadata),
            patch.object(release, "fetch", side_effect=[missing, self.archive]),
        ):
            self.verify()
        self.assertEqual(self.elapsed, 10)
        self.assertEqual((self.download / self.name).read_bytes(), self.archive)

    def test_metadata_and_archive_conflicts_fail_immediately(self):
        for conflict in ("metadata", "archive"):
            with self.subTest(conflict=conflict):
                metadata = json.loads(json.dumps(self.metadata))
                if conflict == "metadata":
                    metadata["dist"]["integrity"] = "sha512-different"
                with (
                    patch.object(release, "registry_metadata", return_value=metadata),
                    patch.object(release, "fetch", return_value=b"different archive"),
                    self.assertRaisesRegex(ValueError, "different bytes|differs"),
                ):
                    self.verify()
                self.assertFalse(self.download.exists())
                self.assertEqual(self.sleeps, [])

    def test_registry_errors_are_not_treated_as_pending_publication(self):
        for status in (401, 403, 429, 500):
            error = release.HTTPError(
                "https://registry.npmjs.org", status, "error", {}, None
            )
            for operation in ("registry_metadata", "fetch"):
                with (
                    self.subTest(status=status, operation=operation),
                    patch.object(
                        release, "registry_metadata", return_value=self.metadata
                    ),
                ):
                    with (
                        patch.object(release, operation, side_effect=error),
                        self.assertRaises(release.HTTPError),
                    ):
                        self.verify()
                self.assertEqual(self.sleeps, [])


if __name__ == "__main__":
    unittest.main()
