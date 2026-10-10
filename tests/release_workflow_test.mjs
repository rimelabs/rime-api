import assert from "node:assert/strict";
import { execFileSync, spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";

const workflow = readFileSync(process.argv[2], "utf8");
const script = workflow.split("          script: |\n")[1]
  .split("\n")
  .map((line) => line.slice(12))
  .join("\n");
const AsyncFunction = Object.getPrototypeOf(async function () {}).constructor;
const dispatchChecks = new AsyncFunction("github", "context", script);
const repository = { owner: "rimelabs", repo: "rime-api" };
const releasePullRequest = {
  number: 42,
  user: { login: "github-actions[bot]" },
  head: {
    repo: { full_name: "rimelabs/rime-api" },
    ref: "release-please--branches--main",
  },
  labels: [{ name: "autorelease: pending" }],
};

async function dispatch(pullRequests) {
  const requests = [];
  const github = {
    rest: {
      pulls: { list() {} },
      actions: {
        async createWorkflowDispatch(request) {
          requests.push(request);
        },
      },
    },
    async paginate(method, request) {
      assert.equal(method, github.rest.pulls.list);
      assert.deepEqual(request, { ...repository, state: "open", base: "main" });
      return pullRequests;
    },
  };
  await dispatchChecks(github, { repo: repository });
  return requests;
}

test("checks run on the release branch, including unchanged PR retries", async () => {
  const expected = [{
    ...repository,
    workflow_id: "check.yaml",
    ref: releasePullRequest.head.ref,
  }, {
    ...repository,
    workflow_id: "schema-source.yaml",
    ref: "main",
    inputs: { pull_request: "42" },
  }];
  assert.deepEqual(await dispatch([releasePullRequest]), expected);
  assert.deepEqual(await dispatch([releasePullRequest]), expected);
});

test("ordinary, fork, and completed PRs do not dispatch release checks", async () => {
  assert.deepEqual(await dispatch([]), []);
  for (const change of [
    { user: { login: "contributor" } },
    { labels: [{ name: "autorelease: tagged" }] },
    { head: { ...releasePullRequest.head, ref: "sync/public-api" } },
    { head: { ...releasePullRequest.head, repo: { full_name: "fork/rime-api" } } },
  ]) {
    assert.deepEqual(await dispatch([{ ...releasePullRequest, ...change }]), []);
  }
});

test("Rust release metadata pushes without an upstream and retries without a new commit", () => {
  const step = workflow.split("      - name: Update Rust release metadata\n")[1]
    .split("      - name: Start checks")[0];
  const shell = step.split("        run: |\n")[1]
    .split("\n").map((line) => line.slice(10)).join("\n");
  const directory = mkdtempSync(join(tmpdir(), "release-metadata-"));
  const remote = join(directory, "remote.git");
  const checkout = join(directory, "checkout");
  const binaries = join(directory, "bin");
  const branch = "release-please--branches--main";
  const environment = {
    ...process.env,
    PATH: `${binaries}:${process.env.PATH}`,
    GIT_AUTHOR_NAME: "Release test",
    GIT_AUTHOR_EMAIL: "test@example.com",
    GIT_COMMITTER_NAME: "Release test",
    GIT_COMMITTER_EMAIL: "test@example.com",
    GIT_CONFIG_GLOBAL: "/dev/null",
    GIT_CONFIG_NOSYSTEM: "1",
  };
  const git = (...args) => execFileSync("git", args, {
    cwd: checkout, env: environment, encoding: "utf8", stdio: ["ignore", "pipe", "pipe"],
  }).trim();
  const update = () => {
    const result = spawnSync("bash", ["-euo", "pipefail", "-c", shell], {
      cwd: checkout, env: environment, encoding: "utf8",
    });
    assert.equal(result.status, 0, result.stdout + result.stderr);
  };
  try {
    mkdirSync(checkout);
    mkdirSync(binaries);
    mkdirSync(join(checkout, "rust"));
    writeFileSync(join(binaries, "gh"), `#!/bin/sh\nprintf '%s\\n' '${branch}'\n`, { mode: 0o755 });
    writeFileSync(join(binaries, "bazel"), "#!/bin/sh\nprintf '%s\\n' 'updated' > rust/SOURCE.json\n", { mode: 0o755 });
    for (const path of ["rust/Cargo.toml", "rust/SOURCE.json", "Cargo.lock", "MODULE.bazel.lock"]) {
      writeFileSync(join(checkout, path), "initial\n");
    }
    git("init", "--bare", remote);
    git("init", "--initial-branch=main");
    git("add", ".");
    git("commit", "-m", "Initial release branch");
    const initial = git("rev-parse", "HEAD");
    git("remote", "add", "origin", remote);
    git("push", "origin", `HEAD:refs/heads/${branch}`);
    update();
    const updated = git("rev-parse", "HEAD");
    assert.notEqual(updated, initial);
    assert.equal(git("rev-parse", "HEAD^"), initial);
    assert.equal(git("--git-dir", remote, "rev-parse", `refs/heads/${branch}`), updated);
    assert.equal(git("--git-dir", remote, "show", `${branch}:rust/SOURCE.json`), "updated");
    update();
    assert.equal(git("rev-parse", "HEAD"), updated);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});
