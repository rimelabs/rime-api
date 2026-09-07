import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { fromBinary, fromJson, toBinary, toJson } from "@bufbuild/protobuf";

const [packages, fixturePath] = process.argv.slice(2);
const directory = resolve(packages, "javascript");
const esm = await import(pathToFileURL(resolve(directory, "esm/index.js")));
const require = createRequire(import.meta.url);
const commonjs = require(resolve(directory, "commonjs/index.js"));
const fixtures = JSON.parse(await readFile(fixturePath, "utf8"));

for (const definitions of [esm, commonjs]) {
  for (const fixture of fixtures) {
    const schema = definitions[`${fixture.message}Schema`];
    const message = fromJson(schema, fixture.json);
    assert.equal(
      Buffer.from(toBinary(schema, message)).toString("hex"),
      fixture.hex,
    );
    const decoded = fromBinary(schema, Buffer.from(fixture.hex, "hex"));
    assert.deepEqual(toJson(schema, decoded), fixture.json);
  }
  assert.equal(
    definitions.WebSocketRequestSchema.typeName,
    "rime.WebSocketRequest",
  );
  assert.equal(definitions.TextToSpeech.typeName, "rime.TextToSpeech");
  const wire = Buffer.from("220568656c6c6ff80701", "hex");
  const decoded = fromBinary(definitions.WebSocketRequestSchema, wire);
  assert.equal(decoded.payload.value, "hello");
  assert.deepEqual(
    Buffer.from(toBinary(definitions.WebSocketRequestSchema, decoded)),
    wire,
  );
}
console.log("ESM and CommonJS pass the shared binary and JSON fixtures.");
