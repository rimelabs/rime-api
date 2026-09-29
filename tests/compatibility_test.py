"""Use the real Buf CLI to distinguish API changes from invalid schemas."""

from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

from tools.check_compatibility import compare

BUF = Path(sys.argv.pop(1)).resolve()
WORKSPACE = Path(sys.argv.pop(1)).resolve()


class CompatibilityTest(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.current = self.root / "current"
        self.baseline = self.root / "baseline"
        for workspace in (self.current, self.baseline):
            shutil.copytree(WORKSPACE, workspace, copy_function=shutil.copyfile)
            (workspace / "schema/rime/text_to_speech.proto").write_text(
                'syntax = "proto3"; package rime; message Request { string text = 1; }\n'
            )

    def write_current(self, content):
        (self.current / "schema/rime/text_to_speech.proto").write_text(content)

    def test_identical_schema_has_no_report(self):
        self.assertEqual(compare(BUF, self.current, self.baseline), [])

    def test_deleted_field_is_reported_without_failure(self):
        self.write_current(
            'syntax = "proto3"; package rime; message Request { reserved 1; }\n'
        )
        diagnostics = compare(BUF, self.current, self.baseline)
        self.assertIn("FIELD_NO_DELETE", [item["type"] for item in diagnostics])

    def test_google_imports_resolve(self):
        self.write_current(
            'syntax = "proto3"; package rime; import "google/rpc/status.proto"; import "google/protobuf/duration.proto"; message Request { string text = 1; google.rpc.Status status = 2; google.protobuf.Duration start = 3; }\n'
        )
        self.assertEqual(compare(BUF, self.current, self.baseline), [])

    def test_missing_import_and_invalid_schema_fail(self):
        for content in ('syntax = "proto3"; import "missing.proto";', "invalid schema"):
            with self.subTest(content=content):
                self.write_current(content)
                with self.assertRaises(subprocess.CalledProcessError):
                    compare(BUF, self.current, self.baseline)

    def test_invalid_baseline_fails(self):
        (self.baseline / "schema/rime/text_to_speech.proto").write_text(
            "invalid schema"
        )
        with self.assertRaises(subprocess.CalledProcessError):
            compare(BUF, self.current, self.baseline)

    def test_operational_error_is_not_a_compatibility_report(self):
        for status in (1, 100):
            with (
                self.subTest(status=status),
                patch("tools.check_compatibility.subprocess.run") as run,
            ):
                run.return_value = subprocess.CompletedProcess(
                    [], status, "", "tool failure"
                )
                with self.assertRaises(RuntimeError):
                    compare(BUF, self.current, self.baseline)


if __name__ == "__main__":
    unittest.main()
