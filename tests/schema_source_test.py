"""Validate schema provenance using the same read-only checker as the trusted job."""

from pathlib import Path
import sys
import unittest
from unittest.mock import patch

from tools import check_schema_source as checker


class SchemaSourceTest(unittest.TestCase):
    def setUp(self):
        self.configuration = configuration
        self.paths, _ = checker.export_paths(self.configuration)
        self.candidate = "a" * 40
        self.source = "b" * 40
        self.files = {
            (
                checker.DESTINATION,
                self.candidate,
                "copy.bara.sky",
            ): self.configuration.encode(),
            (
                checker.DESTINATION,
                self.candidate,
                checker.REVISION_FILE,
            ): self.source.encode(),
        }
        for origin, destination in self.paths.items():
            self.files[checker.SOURCE, self.source, origin] = b"authoritative schema\n"
            self.files[checker.DESTINATION, self.candidate, destination] = (
                b"authoritative schema\n"
            )
        self.source_reads = []

    def read(self, repository, revision, path):
        if repository == checker.SOURCE:
            self.source_reads.append(path)
        return self.files[repository, revision, path]

    def verify(self, allow=False, ancestor=True):
        return checker.verify(
            self.candidate,
            self.configuration,
            self.read,
            lambda revision: ancestor and revision == self.source,
            allow,
        )

    def test_matching_export_passes_without_commit_message_dependency(self):
        self.assertEqual(self.verify(), 4)

    def test_manual_schema_change_or_stale_baseline_fails(self):
        for origin, destination in self.paths.items():
            with self.subTest(destination=destination):
                key = checker.DESTINATION, self.candidate, destination
                self.files[key] = b"manually changed\n"
                with self.assertRaisesRegex(ValueError, "Schemas differ"):
                    self.verify()
                self.files[key] = b"authoritative schema\n"

    def test_private_branch_revision_is_rejected(self):
        with self.assertRaisesRegex(ValueError, "not an ancestor"):
            self.verify(ancestor=False)
        self.assertEqual(self.source_reads, [])

    def test_export_expansion_requires_explicit_migration(self):
        changed = self.configuration.replace(
            '"interfaces/rime/text_to_speech.proto",',
            '"interfaces/rime/text_to_speech.proto", "interfaces/rime/extra.proto",',
        ).replace(
            '"schema/rime/text_to_speech.proto",',
            '"schema/rime/text_to_speech.proto", "schema/rime/extra.proto",',
        )
        self.files[checker.DESTINATION, self.candidate, "copy.bara.sky"] = (
            changed.encode()
        )
        with self.assertRaisesRegex(ValueError, "Export list changed"):
            self.verify()
        self.assertEqual(self.source_reads, [])
        self.files[checker.SOURCE, self.source, "interfaces/rime/extra.proto"] = (
            b"extra"
        )
        self.files[checker.DESTINATION, self.candidate, "schema/rime/extra.proto"] = (
            b"extra"
        )
        self.assertEqual(self.verify(allow=True), 5)

    def test_migration_cannot_change_workflow_behavior(self):
        changed = self.configuration.replace(
            "check_last_rev_state = True", "check_last_rev_state = False"
        )
        self.files[checker.DESTINATION, self.candidate, "copy.bara.sky"] = (
            changed.encode()
        )
        with self.assertRaisesRegex(ValueError, "trusted policy review"):
            self.verify(allow=True)
        self.assertEqual(self.source_reads, [])

    def test_nonliteral_paths_cannot_execute_code(self):
        changed = self.configuration.replace(
            '"interfaces/rime/text_to_speech.proto"', "dangerous_call()"
        )
        self.files[checker.DESTINATION, self.candidate, "copy.bara.sky"] = (
            changed.encode()
        )
        with self.assertRaises(ValueError):
            self.verify()
        self.assertEqual(self.source_reads, [])

    def test_candidate_symlink_is_rejected_before_reading_content(self):
        with (
            patch.object(
                checker,
                "candidate_tree",
                return_value={"schema/file.proto": {"mode": "120000", "type": "blob"}},
            ),
            patch.object(checker, "api") as request,
        ):
            with self.assertRaisesRegex(ValueError, "regular non-executable"):
                checker.read_file(
                    checker.DESTINATION, self.candidate, "schema/file.proto"
                )
            request.assert_not_called()

    def test_invalid_revision_is_rejected_before_source_access(self):
        self.files[checker.DESTINATION, self.candidate, checker.REVISION_FILE] = b"main"
        with self.assertRaisesRegex(ValueError, "full commit ID"):
            self.verify()
        self.assertEqual(self.source_reads, [])


if __name__ == "__main__":
    configuration = Path(sys.argv.pop(1)).read_text()
    unittest.main()
