import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
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
