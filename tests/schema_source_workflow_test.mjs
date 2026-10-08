import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {test} from "node:test";

const workflow = readFileSync(process.argv[2], "utf8");
const script = workflow.match(/          script: \|\n([\s\S]*?)(?=^      -)/m)[1]
  .split("\n").map(line => line.slice(12)).join("\n");
const AsyncFunction = Object.getPrototypeOf(async function () {}).constructor;
const select = new AsyncFunction("github", "context", "core", "process", script);

async function run({fork = false, dispatch = false, requested = "", closed = false} = {}) {
  const statuses = [], outputs = {};
  const github = {rest: {
    pulls: {async get() { return {data: {
      state: closed ? "closed" : "open", base: {ref: "main"},
      head: {sha: "a".repeat(40), repo: {full_name: fork ? "outside/rime-api" : "rimelabs/rime-api"}},
    }}; }},
    repos: {async createCommitStatus(status) {statuses.push(status);}},
  }};
  const context = {
    repo: {owner: "rimelabs", repo: "rime-api"},
    payload: dispatch ? {} : {pull_request: {number: 42}},
    eventName: dispatch ? "workflow_dispatch" : "pull_request_target",
    sha: "b".repeat(40), serverUrl: "https://github.com", runId: 1,
  };
  let error;
  try {
    await select(github, context, {setOutput(name,value) {outputs[name]=value;}}, {env: {REQUESTED_PR: requested}});
  } catch (caught) {error = caught;}
  return {statuses, outputs, error};
}

test("fork events stop before the credentialed step, while maintainer dispatch can proceed", async () => {
  const blocked = await run({fork: true});
  assert.match(blocked.error.message, /maintainer must dispatch/);
  assert.equal(blocked.outputs.sha, "a".repeat(40));
  assert.equal(blocked.statuses[0].state, "pending");
  assert.equal((await run({fork: true, dispatch: true, requested: "42"})).error, undefined);
});

test("same repository checks use the current PR head, not the event base SHA", async () => {
  const result = await run();
  assert.equal(result.error, undefined);
  assert.equal(result.statuses[0].sha, "a".repeat(40));
});

test("invalid dispatch and closed PRs fail without posting a status", async () => {
  for (const options of [{dispatch: true, requested: "not-a-number"}, {closed: true}]) {
    const result = await run(options);
    assert.ok(result.error);
    assert.deepEqual(result.statuses, []);
  }
});
