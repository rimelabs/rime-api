import assert from "node:assert/strict";
import { execFileSync, spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { basename, join } from "node:path";
import { test } from "node:test";

const workflow = readFileSync(process.argv[2], "utf8");
const concurrency = workflow.match(/^concurrency:\n((?:[ \t]+.*\n)+)/m)[1];
const group = concurrency.match(/^  group: (.+)$/m)[1];
const selectionWorkflow = readFileSync(process.argv[3], "utf8");
const script = selectionWorkflow
  .split("          script: |\n")[1]
  .split("\n")
  .map((line) => line.slice(12))
  .join("\n");
const AsyncFunction = Object.getPrototypeOf(async function () {}).constructor;
const dispatchPublication = new AsyncFunction(
  "github", "context", "process", script,
);

function publicationGroup(runId, version) {
  const values = {
    "inputs.run_id": runId,
    "inputs.version": version,
  };
  return group.replace(/\$\{\{(.*?)\}\}/g, (_, expression) => {
    const alternatives = expression.split("||").map((name) => name.trim());
    assert.ok(
      alternatives.every((name) => name in values),
      "Unknown concurrency input",
    );
    return alternatives.map((name) => values[name]).find(Boolean) ?? "";
  });
}

test("separate checks for the same version share the complete publication lock", async () => {
  const requests = [];
  const repository = { owner: "rimelabs", repo: "rime-api" };
  const github = {
    rest: {
      actions: {
        async createWorkflowDispatch(request) {
          requests.push(request);
        },
      },
    },
  };
  for (const runId of ["100", "101"]) {
    await dispatchPublication(github, { repo: repository }, {
      env: {
        RELEASE_RUN_ID: runId,
        RELEASE_VERSION: "0.2.0",
        RELEASE_REGISTRY: "both",
      },
    });
  }
  assert.deepEqual(requests, ["100", "101"].map((runId) => ({
    ...repository,
    workflow_id: "release.yaml",
    ref: "main",
    inputs: { run_id: runId, version: "0.2.0", registry: "both" },
  })));
  const groups = requests.map(({ inputs }) =>
    publicationGroup(inputs.run_id, inputs.version),
  );
  assert.equal(groups[0], groups[1]);
  assert.equal(groups[0], publicationGroup("102", "0.2.0"));
  assert.match(concurrency, /^  cancel-in-progress: false$/m);
  // The lock is at workflow level and covers registry uploads and finalization.
  assert.ok(workflow.indexOf("concurrency:") < workflow.indexOf("jobs:"));
  assert.doesNotMatch(workflow, /^  workflow_run:/m);
});

test("different versions can publish independently", () => {
  assert.notEqual(
    publicationGroup(100, "0.2.0"), publicationGroup(101, "0.3.0"),
  );
});

test("finalization selects the tested Go version and supports releases before Go", () => {
  const step = workflow.split("      - name: Select the tested Go toolchain\n")[1]
    .split("      - uses:")[0];
  const shell = step.split("        run: |\n")[1]
    .split("\n").map((line) => line.slice(10)).join("\n");
  const setup = workflow.split("      - uses: actions/setup-go@v6\n")[1]
    .split("      - name:")[0];
  assert.match(setup, /steps\.go_toolchain\.outputs\.version_file != ''/);
  assert.match(setup, /go-version-file: \$\{\{ steps\.go_toolchain\.outputs\.version_file \}\}/);
  assert.match(setup, /cache: false/);

  const directory = mkdtempSync(join(tmpdir(), "release-toolchain-"));
  const environment = {
    ...process.env,
    GIT_AUTHOR_NAME: "Release test",
    GIT_AUTHOR_EMAIL: "test@example.com",
    GIT_COMMITTER_NAME: "Release test",
    GIT_COMMITTER_EMAIL: "test@example.com",
    RUNNER_TEMP: directory,
    GITHUB_OUTPUT: join(directory, "output"),
  };
  const git = (...args) => execFileSync("git", args, {
    cwd: directory, env: environment, encoding: "utf8", stdio: ["ignore", "pipe", "pipe"],
  }).trim();
  const select = (commit) => {
    writeFileSync(join(directory, "release-dist/RELEASE.json"), JSON.stringify({ commit }));
    writeFileSync(environment.GITHUB_OUTPUT, "");
    return spawnSync("bash", ["-euo", "pipefail", "-c", shell], {
      cwd: directory, env: environment, encoding: "utf8",
    });
  };
  try {
    git("init");
    git("-c", "commit.gpgsign=false", "commit", "--allow-empty", "-m", "Before Go");
    const beforeGo = git("rev-parse", "HEAD");
    mkdirSync(join(directory, "go"));
    const testedModule = "module example.com/protocol\n\ngo 1.24.0\n";
    writeFileSync(join(directory, "go/go.mod"), testedModule);
    git("add", "go/go.mod");
    git("-c", "commit.gpgsign=false", "commit", "-m", "Tested Go release");
    const testedCommit = git("rev-parse", "HEAD");
    writeFileSync(join(directory, "go/go.mod"), "module example.com/protocol\n\ngo 1.25.0\n");
    git("add", "go/go.mod");
    git("-c", "commit.gpgsign=false", "commit", "-m", "Later Go version");
    mkdirSync(join(directory, "release-dist"));

    const selected = select(testedCommit);
    assert.equal(selected.status, 0, selected.stderr);
    const versionFile = readFileSync(environment.GITHUB_OUTPUT, "utf8").trim().split("=")[1];
    // actions/setup-go parses module syntax only for this exact basename.
    assert.equal(basename(versionFile), "go.mod");
    assert.equal(readFileSync(join(directory, "release-go/go.mod"), "utf8"), testedModule);
    assert.equal(readFileSync(environment.GITHUB_OUTPUT, "utf8"), `version_file=${directory}/release-go/go.mod\n`);

    const legacy = select(beforeGo);
    assert.equal(legacy.status, 0, legacy.stderr);
    assert.equal(readFileSync(environment.GITHUB_OUTPUT, "utf8"), "");
    assert.notEqual(select("HEAD").status, 0);
    assert.notEqual(select("0".repeat(40)).status, 0);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});
