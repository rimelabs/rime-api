"""Exercise schema export against temporary local Git repositories."""

import argparse
from pathlib import Path
import subprocess
import tempfile
import unittest


def run(*arguments, directory=None, expected=(0,)):
    result = subprocess.run(
        arguments,
        cwd=directory,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
    )
    if result.returncode not in expected:
        raise AssertionError(
            f"{arguments[0]} exited {result.returncode}:\n{result.stdout}"
        )
    return result.stdout


def initialize(directory):
    directory.mkdir()
    run("git", "init", "--initial-branch=main", str(directory))
    run("git", "config", "user.name", "Test Author", directory=directory)
    run("git", "config", "user.email", "test@example.invalid", directory=directory)


def commit(directory, message):
    run("git", "add", ".", directory=directory)
    run("git", "commit", "-m", message, directory=directory)
    return run("git", "rev-parse", "HEAD", directory=directory).strip()


class CopybaraTest(unittest.TestCase):
    def test_bootstrap_from_recorded_source_revision(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            origin = root / "origin"
            destination = root / "destination"
            initialize(origin)
            initialize(destination)
            for repository, directory in (
                (origin, "interfaces"),
                (destination, "schema"),
            ):
                (repository / directory / "rime").mkdir(parents=True)
                (repository / directory / "rime/text_to_speech.proto").write_text(
                    'syntax = "proto3";\n'
                )
                (repository / directory / "text_to_speech.asyncapi.yaml").write_text(
                    "asyncapi: 3.0.0\n"
                )
            baseline = commit(origin, "Initial source")
            commit(destination, "Initial public snapshot without a revision trailer")
            configuration = root / "copy.bara.sky"
            configuration.write_text(
                arguments.configuration.read_text()
                .replace("https://github.com/rimelabs/rime.git", origin.as_uri())
                .replace(
                    "https://github.com/rimelabs/rime-api.git", destination.as_uri()
                )
            )
            command = (
                str(arguments.copybara),
                "migrate",
                str(configuration),
                "public_api",
                "--git-committer-name=Test Sync",
                "--git-committer-email=test@example.invalid",
                "--git-destination-non-fast-forward",
                f"--last-rev={baseline}",
            )
            run(*command, expected=(4,))
            (origin / "unrelated.txt").write_text("No public API change\n")
            commit(origin, "Change only an unrelated file")
            run(*command, expected=(4,))
            self.assertEqual(
                run(
                    "git", "branch", "--list", "sync/public-api", directory=destination
                ),
                "",
            )
            (origin / "interfaces/rime/text_to_speech.proto").write_text(
                'syntax = "proto3";\nmessage Added {}\n'
            )
            revision = commit(origin, "Add public message")
            run(*command)
            message = run(
                "git",
                "log",
                "-1",
                "--format=%B",
                "sync/public-api",
                directory=destination,
            )
            self.assertIn(f"GitOrigin-RevId: {revision}", message)
            exported = run(
                "git",
                "show",
                "sync/public-api:schema/rime/text_to_speech.proto",
                directory=destination,
            )
            self.assertIn("message Added", exported)
            # Repeating an export before merge must keep the pending change.
            run(*command, expected=(0,))
            run("git", "revert", "--no-edit", revision, directory=origin)
            run(*command, expected=(4,))
            # Copybara leaves the stale branch for the workflow to remove.
            self.assertIn(
                "message Added",
                run(
                    "git",
                    "show",
                    "sync/public-api:schema/rime/text_to_speech.proto",
                    directory=destination,
                ),
            )

    def test_export_update_repeat_and_delete(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            origin = root / "origin"
            destination = root / "destination"
            initialize(origin)
            initialize(destination)
            (origin / "interfaces/rime/model").mkdir(parents=True)
            (origin / "interfaces/rime/text_to_speech.proto").write_text(
                'syntax = "proto3";\n'
            )
            (origin / "interfaces/text_to_speech.asyncapi.yaml").write_text(
                "asyncapi: 3.0.0\n"
            )
            (origin / "interfaces/rime/model/model.proto").write_text(
                "INTERNAL MODEL\n"
            )
            (origin / "interfaces/new_private_file.txt").write_text("INTERNAL FILE\n")
            first_revision = commit(origin, "PRIVATE COMMIT MESSAGE")
            (destination / "README.md").write_text("Destination-owned documentation\n")
            (destination / "schema").mkdir()
            (destination / "schema/BUILD.bazel").write_text(
                "# Destination-owned build rules\n"
            )
            commit(destination, "Initialize destination")
            configuration = root / "copy.bara.sky"
            configuration.write_text(
                arguments.configuration.read_text()
                .replace("https://github.com/rimelabs/rime.git", origin.as_uri())
                .replace(
                    "https://github.com/rimelabs/rime-api.git", destination.as_uri()
                )
            )

            def migrate(*options, expected=(0,)):
                return run(
                    str(arguments.copybara),
                    "migrate",
                    str(configuration),
                    "public_api",
                    "--git-committer-name=Test Sync",
                    "--git-committer-email=test@example.invalid",
                    "--git-destination-non-fast-forward",
                    *options,
                    expected=expected,
                )

            def exported(path):
                return run(
                    "git", "show", f"sync/public-api:{path}", directory=destination
                )

            migrate("--init-history")
            files = run(
                "git",
                "ls-tree",
                "-r",
                "--name-only",
                "sync/public-api",
                directory=destination,
            ).splitlines()
            self.assertEqual(
                set(files),
                {
                    "README.md",
                    "schema/BUILD.bazel",
                    "schema/rime/text_to_speech.proto",
                    "schema/text_to_speech.asyncapi.yaml",
                },
            )
            message = run(
                "git",
                "log",
                "-1",
                "--format=%B",
                "sync/public-api",
                directory=destination,
            )
            self.assertIn(f"GitOrigin-RevId: {first_revision}", message)
            self.assertNotIn("PRIVATE COMMIT MESSAGE", message)
            self.assertEqual(
                exported("schema/rime/text_to_speech.proto"), 'syntax = "proto3";\n'
            )
            self.assertEqual(exported("README.md"), "Destination-owned documentation\n")
            run("git", "merge", "--ff-only", "sync/public-api", directory=destination)
            migrated_revision = run(
                "git", "rev-parse", "sync/public-api", directory=destination
            )
            migrate(expected=(0, 4))
            self.assertEqual(
                run("git", "rev-parse", "sync/public-api", directory=destination),
                migrated_revision,
            )

            (origin / "interfaces/rime/text_to_speech.proto").write_text(
                'syntax = "proto3";\nmessage Added {}\n'
            )
            (origin / "interfaces/text_to_speech.asyncapi.yaml").unlink()
            revision = commit(origin, "ANOTHER PRIVATE MESSAGE")
            migrate()
            self.assertIn("message Added", exported("schema/rime/text_to_speech.proto"))
            files = run(
                "git",
                "ls-tree",
                "-r",
                "--name-only",
                "sync/public-api",
                directory=destination,
            ).splitlines()
            self.assertNotIn("schema/text_to_speech.asyncapi.yaml", files)
            self.assertIn("schema/BUILD.bazel", files)
            migrate(expected=(0,))
            run("git", "revert", "--no-edit", revision, directory=origin)
            migrate(expected=(4,))
            self.assertIn("message Added", exported("schema/rime/text_to_speech.proto"))


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--copybara", type=Path, required=True)
    parser.add_argument("--configuration", type=Path, required=True)
    arguments = parser.parse_args()
    arguments.copybara = arguments.copybara.resolve()
    arguments.configuration = arguments.configuration.resolve()
    unittest.main(argv=["copybara_test"])
