import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
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
