"""Run the workflow shell blocks with local Git and a fake GitHub CLI."""

import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile
import textwrap
import unittest


class SyncWorkflowTest(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.repository = self.root / "repository"
        self.remote = self.root / "remote.git"
        self.commands = self.root / "commands"
        self.commands.mkdir()
        self.pull_requests = self.root / "pull_requests"
        self.pull_requests.write_text("17\n")
        self.output = self.root / "output"
        self.output.touch()
        self.pushed_commits = self.remote / "pushed-commits"
        self.environment = dict(
            os.environ,
            PATH=f"{self.commands}{os.pathsep}{os.environ['PATH']}",
            REAL_GIT=shutil.which("git"),
            PULL_REQUESTS=str(self.pull_requests),
            GITHUB_OUTPUT=str(self.output),
            RUNNER_TEMP=str(self.root),
        )
        self.install_command(
            "gh",
            """
            case "$1 $2" in
              'pr list')
                [[ "$*" == *'--head sync/public-api --base main --state open'* ]]
                if [[ "${PR_LIST_STATUS:-0}" != 0 ]]; then exit "$PR_LIST_STATUS"; fi
                if [[ "${!#}" == length ]]; then
                  if [[ -s "$PULL_REQUESTS" ]]; then echo 1; else echo 0; fi
                else
                  cat "$PULL_REQUESTS"
                fi
                ;;
              'pr close')
                if [[ "${PR_CLOSE_STATUS:-0}" != 0 ]]; then exit "$PR_CLOSE_STATUS"; fi
                [[ "$3" == "$(cat "$PULL_REQUESTS")" ]]
                : > "$PULL_REQUESTS"
                ;;
              'pr create') echo '18' > "$PULL_REQUESTS" ;;
              *) exit 99 ;;
            esac
            """,
        )
        self.install_command(
            "bazel",
            """
            if [[ "$*" == 'run //:update_go' ]]; then
              if [[ "${GO_STATUS:-0}" != 0 ]]; then exit "$GO_STATUS"; fi
              mkdir -p go
              echo generated > go/definition.pb.go
              exit 0
            fi
            if [[ "$*" == 'run //:update_rust' ]]; then
              if [[ "${RUST_STATUS:-0}" != 0 ]]; then exit "$RUST_STATUS"; fi
              mkdir -p rust/src/generated
              echo generated > rust/src/generated/rime.rs
              if [[ -n "${CONCURRENT_COMMIT:-}" ]]; then
                "$REAL_GIT" push --force origin "$CONCURRENT_COMMIT:refs/heads/sync/public-api"
              fi
              exit 0
            fi
            if [[ "${COPYBARA_STATUS:-0}" == 4 && -n "${CONCURRENT_COMMIT:-}" ]]; then
              "$REAL_GIT" push --force origin "$CONCURRENT_COMMIT:refs/heads/sync/public-api"
            fi
            if [[ "${COPYBARA_STATUS:-0}" != 0 ]]; then exit "$COPYBARA_STATUS"; fi
            destination=origin
            for argument in "$@"; do
              case "$argument" in
                --git-destination-url=*) destination="${argument#*=}" ;;
              esac
            done
            "$REAL_GIT" push --force "$destination" "$EXPORTED_COMMIT:refs/heads/sync/public-api"
            """,
        )
        self.install_command(
            "git",
            """
            if [[ "$1" == ls-remote && "${REMOTE_STATUS:-0}" != 0 ]]; then
              exit "$REMOTE_STATUS"
            fi
            if [[ "$1" == push && "${PUSH_STATUS:-0}" != 0 ]]; then
              exit "$PUSH_STATUS"
            fi
            if [[ "$1" == push && "$*" == *'--delete sync/public-api'* && -n "${CONCURRENT_DELETE_COMMIT:-}" ]]; then
              "$REAL_GIT" push --force origin "$CONCURRENT_DELETE_COMMIT:refs/heads/sync/public-api"
            fi
            exec "$REAL_GIT" "$@"
            """,
        )
        self.git("init", "--bare", "--initial-branch=main", str(self.remote))
        self.git("clone", str(self.remote), str(self.repository))
        self.git("config", "user.name", "Test Sync")
        self.git("config", "user.email", "test@example.invalid")
        (self.repository / "sync").mkdir()
        (self.repository / "sync/SOURCE_REVISION").write_text("a" * 40 + "\n")
        (self.repository / "schema").mkdir()
        self.schema = self.repository / "schema" / "definition"
        self.schema.write_text("baseline\n")
        self.git("add", ".")
        self.git("commit", "-m", "Initial snapshot\n\nGitOrigin-RevId: baseline")
        self.git("push", "origin", "main")
        self.schema.write_text("pending change\n")
        self.git("commit", "-am", "Pending export\n\nGitOrigin-RevId: " + "b" * 40)
        self.environment["EXPORTED_COMMIT"] = self.git("rev-parse", "HEAD").strip()
        self.git("branch", "export-fixture")
        self.schema.write_text("previous pending change\n")
        (self.repository / "sync/SOURCE_REVISION").write_text("c" * 40 + "\n")
        (self.repository / "go").mkdir()
        (self.repository / "go/definition.pb.go").write_text("previous\n")
        (self.repository / "rust/src/generated").mkdir(parents=True)
        (self.repository / "rust/src/generated/rime.rs").write_text("previous\n")
        self.git("add", ".")
        self.git("commit", "--amend", "-m", "Previous complete export")
        self.git("push", "origin", "HEAD:sync/public-api")
        self.git("reset", "--hard", "origin/main")
        # Record every commit published to the PR branch, including intermediate pushes.
        hook = self.remote / "hooks/post-receive"
        hook.write_text(
            "#!/usr/bin/env bash\n"
            "while read -r previous current reference; do\n"
            '  if [[ "$reference" == refs/heads/sync/public-api ]]; then\n'
            '    echo "$current" >> pushed-commits\n'
            "  fi\n"
            "done\n"
        )
        hook.chmod(0o755)

    def install_command(self, name, script):
        path = self.commands / name
        path.write_text(
            "#!/usr/bin/env bash\nset -euo pipefail\n" + textwrap.dedent(script)
        )
        path.chmod(0o755)

    def git(self, *arguments):
        return subprocess.run(
            [self.environment["REAL_GIT"], *arguments],
            cwd=self.repository if self.repository.exists() else self.root,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=True,
        ).stdout

    def run_step(self, name):
        # Extract literal shell blocks so tests execute the workflow's own code.
        block = steps[name]
        script = textwrap.dedent(block.split("        run: |\n", 1)[1])
        return subprocess.run(
            ["bash", "--noprofile", "--norc", "-e", "-o", "pipefail", "-c", script],
            cwd=self.repository,
            env=self.environment,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
        )

    def run_sync_steps(self, status):
        self.environment["COPYBARA_STATUS"] = str(status)
        self.output.write_text("")
        result = self.run_step("Export schemas with Copybara")
        if result.returncode:
            return result
        outputs = dict(
            line.split("=", 1) for line in self.output.read_text().splitlines()
        )
        for name in ("Close the stale schema PR", "Open or retain the schema PR"):
            condition = re.search(
                r"if: steps\.export\.outputs\.status == '(\d+)'", steps[name]
            )
            self.assertIsNotNone(condition)
            if outputs["status"] == condition[1]:
                for variable, output in re.findall(
                    r"^          (\w+): \$\{\{ steps\.export\.outputs\.(\w+) \}\}$",
                    steps[name],
                    re.M,
                ):
                    self.environment[variable] = outputs[output]
                result = self.run_step(name)
                if result.returncode:
                    return result
        return result

    def branch_exists(self):
        return bool(
            self.git("ls-remote", "--heads", "origin", "refs/heads/sync/public-api")
        )

    def test_no_changes_closes_pr_deletes_branch_and_can_repeat(self):
        for _ in range(2):
            result = self.run_sync_steps(4)
            self.assertEqual(result.returncode, 0, result.stdout)
            self.assertEqual(self.pull_requests.read_text(), "")
            self.assertFalse(self.branch_exists())

    def test_pending_change_retains_pr_and_branch(self):
        result = self.run_sync_steps(0)
        self.assertEqual(result.returncode, 0, result.stdout)
        self.assertEqual(self.pull_requests.read_text(), "17\n")
        self.assertTrue(self.branch_exists())
        self.assertEqual(
            self.git("show", "origin/sync/public-api:go/definition.pb.go").strip(),
            "generated",
        )
        self.assertEqual(
            self.git("show", "origin/sync/public-api:sync/SOURCE_REVISION").strip(),
            "b" * 40,
        )

    def test_only_complete_commit_is_pushed(self):
        result = self.run_sync_steps(0)
        self.assertEqual(result.returncode, 0, result.stdout)
        published = self.pushed_commits.read_text().splitlines()
        self.assertEqual(len(published), 1, result.stdout)
        for path, expected in (
            ("schema/definition", "pending change"),
            ("sync/SOURCE_REVISION", "b" * 40),
            ("go/definition.pb.go", "generated"),
            ("rust/src/generated/rime.rs", "generated"),
        ):
            self.assertEqual(
                self.git("show", f"{published[0]}:{path}").strip(), expected
            )

    def test_generation_failure_does_not_publish(self):
        for variable in ("GO_STATUS", "RUST_STATUS"):
            with self.subTest(command=variable):
                self.environment[variable] = "1"
                result = self.run_sync_steps(0)
                del self.environment[variable]
                self.assertEqual(result.returncode, 1, result.stdout)
                self.assertFalse(self.pushed_commits.exists(), result.stdout)
                self.assertEqual(self.pull_requests.read_text(), "17\n")
                self.git("reset", "--hard", "origin/main")
                self.git("clean", "-fd")

    def test_first_export_creates_branch_and_pr(self):
        self.git("push", "origin", "--delete", "sync/public-api")
        self.pushed_commits.unlink()
        self.pull_requests.write_text("")
        result = self.run_sync_steps(0)
        self.assertEqual(result.returncode, 0, result.stdout)
        self.assertEqual(self.pull_requests.read_text(), "18\n")
        self.assertTrue(self.branch_exists())
        self.assertEqual(len(self.pushed_commits.read_text().splitlines()), 1)

    def test_concurrent_branch_update_is_preserved(self):
        self.git("switch", "export-fixture")
        self.schema.write_text("concurrent update\n")
        self.git("commit", "-am", "Concurrent update")
        concurrent_commit = self.git("rev-parse", "HEAD").strip()
        self.git("switch", "main")
        self.environment["CONCURRENT_COMMIT"] = concurrent_commit
        result = self.run_sync_steps(0)
        self.assertNotEqual(result.returncode, 0, result.stdout)
        remote_commit = self.git(
            "ls-remote", "--heads", "origin", "refs/heads/sync/public-api"
        ).split()[0]
        self.assertEqual(remote_commit, concurrent_commit)

    def test_publish_failure_preserves_pr_and_branch(self):
        self.environment["PUSH_STATUS"] = "1"
        result = self.run_sync_steps(0)
        self.assertNotEqual(result.returncode, 0, result.stdout)
        self.assertFalse(self.pushed_commits.exists(), result.stdout)
        self.assertEqual(self.pull_requests.read_text(), "17\n")

    def test_stale_cleanup_preserves_concurrent_branch_update(self):
        self.environment["CONCURRENT_COMMIT"] = self.environment["EXPORTED_COMMIT"]
        result = self.run_sync_steps(4)
        self.assertNotEqual(result.returncode, 0, result.stdout)
        remote_commit = self.git(
            "ls-remote", "--heads", "origin", "refs/heads/sync/public-api"
        ).split()[0]
        self.assertEqual(remote_commit, self.environment["CONCURRENT_COMMIT"])
        self.assertEqual(self.pull_requests.read_text(), "17\n")

    def test_stale_cleanup_lease_preserves_update_after_branch_check(self):
        self.environment["CONCURRENT_DELETE_COMMIT"] = self.environment[
            "EXPORTED_COMMIT"
        ]
        result = self.run_sync_steps(4)
        self.assertNotEqual(result.returncode, 0, result.stdout)
        remote_commit = self.git(
            "ls-remote", "--heads", "origin", "refs/heads/sync/public-api"
        ).split()[0]
        self.assertEqual(remote_commit, self.environment["CONCURRENT_DELETE_COMMIT"])
        self.assertEqual(self.pull_requests.read_text(), "17\n")

    def test_pr_close_failure_can_be_retried_after_branch_deletion(self):
        self.environment["PR_CLOSE_STATUS"] = "1"
        result = self.run_sync_steps(4)
        self.assertEqual(result.returncode, 1, result.stdout)
        self.assertFalse(self.branch_exists())
        self.assertEqual(self.pull_requests.read_text(), "17\n")
        del self.environment["PR_CLOSE_STATUS"]
        result = self.run_sync_steps(4)
        self.assertEqual(result.returncode, 0, result.stdout)
        self.assertEqual(self.pull_requests.read_text(), "")

    def test_copybara_failure_preserves_pr_and_branch(self):
        result = self.run_sync_steps(2)
        self.assertEqual(result.returncode, 2, result.stdout)
        self.assertEqual(self.output.read_text(), "")
        self.assertEqual(self.pull_requests.read_text(), "17\n")
        self.assertTrue(self.branch_exists())

    def test_cleanup_errors_fail_the_job(self):
        for variable in (
            "PR_LIST_STATUS",
            "REMOTE_STATUS",
            "PUSH_STATUS",
        ):
            with self.subTest(command=variable):
                self.pull_requests.write_text("17\n")
                self.environment[variable] = "1"
                result = self.run_sync_steps(4)
                del self.environment[variable]
                self.assertEqual(result.returncode, 1, result.stdout)
                self.assertTrue(self.branch_exists())
                self.assertEqual(self.pull_requests.read_text(), "17\n")


if __name__ == "__main__":
    workflow = Path(sys.argv.pop(1)).read_text()
    steps = dict(
        re.findall(
            r"^      - name: ([^\n]+)\n(.*?)(?=^      - |\Z)", workflow, re.M | re.S
        )
    )
    unittest.main()
