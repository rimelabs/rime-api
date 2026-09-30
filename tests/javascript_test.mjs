import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { fromBinary, fromJson, toBinary, toJson } from "@bufbuild/protobuf";
import typescript from "typescript";

const [packages, fixturePath] = process.argv.slice(2);
const directory = resolve(packages, "javascript");
const esm = await import(pathToFileURL(resolve(directory, "esm/index.js")));
const require = createRequire(import.meta.url);
const commonjs = require(resolve(directory, "commonjs/index.js"));
const fixtures = JSON.parse(await readFile(fixturePath, "utf8"));
const source = await readFile(resolve(directory, "schema/rime/text_to_speech.proto"), "utf8");

for (const definitions of [esm, commonjs]) {
  for (const fixture of fixtures) {
    if (fixture.requires && !source.includes(`message ${fixture.requires} {`)) continue;
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
  assert.equal(definitions.SpeechToText.typeName, "rime.SpeechToText");
  assert.equal(definitions.SpeechWebSocketRequestSchema.typeName, "rime.SpeechWebSocketRequest");
  const wire = Buffer.from("220568656c6c6ff80701", "hex");
  const decoded = fromBinary(definitions.WebSocketRequestSchema, wire);
  assert.equal(decoded.payload.value, "hello");
  assert.deepEqual(
    Buffer.from(toBinary(definitions.WebSocketRequestSchema, decoded)),
    wire,
  );
}
console.log("ESM and CommonJS pass the shared binary and JSON fixtures.");

const consumer = `
import { create } from "@bufbuild/protobuf";
import { WebSocketRequestSchema, SpeechWebSocketRequestSchema, StreamingOutputContract } from "@rimelabs/api";
import type { WebSocketRequest, SpeechWebSocketRequest } from "@rimelabs/api";
const synthesis: WebSocketRequest = create(WebSocketRequestSchema, { payload: { case: "text", value: "Hello." } });
const recognition: SpeechWebSocketRequest = create(SpeechWebSocketRequestSchema, {
  payload: { case: "start", value: { language: "en", outputContract: StreamingOutputContract.REVISED_HYPOTHESES } },
});
// @ts-expect-error STT audio must be bytes.
const invalidAudio: SpeechWebSocketRequest = { ...recognition, payload: { case: "audio", value: "audio" } };
// @ts-expect-error TTS text must be a string.
const invalidText: WebSocketRequest = { ...synthesis, payload: { case: "text", value: 42 } };
`;
for (const [format, extension] of [["esm", "mts"], ["commonjs", "cts"]]) {
  const filename = resolve(directory, `consumer.${extension}`);
  const options = {
    strict: true,
    noEmit: true,
    target: typescript.ScriptTarget.ES2022,
    module: typescript.ModuleKind.NodeNext,
    paths: { "@rimelabs/api": [resolve(directory, format, "index.d.ts")] },
  };
  const host = typescript.createCompilerHost(options);
  const getSourceFile = host.getSourceFile.bind(host);
  host.getSourceFile = (path, languageVersion, ...arguments_) => path === filename
    ? typescript.createSourceFile(path, consumer, languageVersion)
    : getSourceFile(path, languageVersion, ...arguments_);
  const program = typescript.createProgram([filename], options, host);
  const diagnostics = typescript.getPreEmitDiagnostics(program);
  assert.equal(diagnostics.length, 0, typescript.formatDiagnosticsWithColorAndContext(diagnostics, {
    getCanonicalFileName: path => path,
    getCurrentDirectory: () => process.cwd(),
    getNewLine: () => "\n",
  }));
}
console.log("ESM and CommonJS pass TypeScript consumer checks.");
