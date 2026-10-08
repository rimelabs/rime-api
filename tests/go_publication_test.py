"""Run public-download verification against a real Git tree and a fake proxy response."""

import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest


class GoPublicationTest(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.repository = self.root / "repository"
        self.repository.mkdir()
        self.environment = dict(
            os.environ,
            GIT_CONFIG_GLOBAL=os.devnull,
            GIT_AUTHOR_NAME="Test",
            GIT_COMMITTER_NAME="Test",
            GIT_AUTHOR_EMAIL="test@example.invalid",
            GIT_COMMITTER_EMAIL="test@example.invalid",
        )
        self.git("init")
        module = self.repository / "go"
        module.mkdir()
        (module / "go.mod").write_text(
            "module github.com/rimelabs/rime-api/go\n\ngo 1.24.0\n"
        )
        (module / "protocol.go").write_text("package rimeapi\n")
        self.git("add", ".")
        self.git("-c", "commit.gpgsign=false", "commit", "-m", "Tested source")
        self.commit = self.git("rev-parse", "HEAD").strip()
        self.download = self.root / "download"
        shutil.copytree(module, self.download)
        self.metadata = self.root / "module.json"
        self.metadata.write_text(
            json.dumps({"Version": "v0.3.0", "Dir": str(self.download)})
        )
        commands = self.root / "commands"
        commands.mkdir()
        command = commands / "go"
        command.write_text(
            '#!/usr/bin/env bash\nset -euo pipefail\nif [[ "$1 $2" == "mod download" ]]; then cat "$TEST_MODULE_METADATA"; fi\n'
        )
        command.chmod(0o755)
        self.environment.update(
            PATH=str(commands) + os.pathsep + os.environ["PATH"],
            TEST_MODULE_METADATA=str(self.metadata),
        )

    def git(self, *arguments):
        return subprocess.check_output(
            ["git", *arguments],
            cwd=self.repository,
            env=self.environment,
            stderr=subprocess.DEVNULL,
            text=True,
        )

    def verify(self):
        return subprocess.run(
            ["bash", str(script), "0.3.0", self.commit],
            cwd=self.repository,
            env=self.environment,
            capture_output=True,
            text=True,
        )

    def test_accepts_identical_files_without_optional_origin_metadata(self):
        result = self.verify()
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_rejects_changed_missing_and_extra_files(self):
        (self.download / "protocol.go").write_text("package modified\n")
        self.assertNotEqual(self.verify().returncode, 0)
        (self.download / "protocol.go").unlink()
        self.assertNotEqual(self.verify().returncode, 0)
        shutil.copyfile(
            self.repository / "go/protocol.go", self.download / "protocol.go"
        )
        (self.download / "extra.go").write_text("package rimeapi\n")
        self.assertNotEqual(self.verify().returncode, 0)

    def test_rejects_another_version(self):
        self.metadata.write_text(
            json.dumps({"Version": "v0.2.0", "Dir": str(self.download)})
        )
        self.assertNotEqual(self.verify().returncode, 0)


if __name__ == "__main__":
    script = Path(sys.argv.pop(1)).resolve()
    unittest.main()
