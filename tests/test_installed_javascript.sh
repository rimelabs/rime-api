#!/usr/bin/env bash
set -euo pipefail

distribution_directory=$(cd "$(dirname "$1")" && pwd)
distribution="$distribution_directory/$(basename "$1")"
test -f "$distribution"
source_directory=$(cd "$(dirname "$0")" && pwd)
consumer=$(mktemp -d)
trap 'rm -rf "$consumer"' EXIT
cd "$consumer"
cat >package.json <<'JSON'
{"name":"rime-api-consumer","private":true,"type":"module"}
JSON
npm install --ignore-scripts --no-audit --no-fund "$distribution" \
	@bufbuild/protobuf@2.11.0 typescript@5.9.3
cp "$source_directory/fixtures.json" fixtures.json
cat >smoke.mjs <<'JS'
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { createRequire } from 'node:module';
import * as esm from '@rimelabs/api';
import { fromBinary, fromJson, toBinary, toJson } from '@bufbuild/protobuf';
const commonjs = createRequire(import.meta.url)('@rimelabs/api');
const source = readFileSync('node_modules/@rimelabs/api/schema/rime/text_to_speech.proto', 'utf8');
for (const definitions of [esm, commonjs]) {
  for (const fixture of JSON.parse(readFileSync('fixtures.json', 'utf8'))) {
    if (fixture.requires && !source.includes(`message ${fixture.requires} {`)) continue;
    const schema = definitions[`${fixture.message}Schema`];
    assert.equal(Buffer.from(toBinary(schema, fromJson(schema, fixture.json))).toString('hex'), fixture.hex);
    assert.deepEqual(toJson(schema, fromBinary(schema, Buffer.from(fixture.hex, 'hex'))), fixture.json);
  }
}
JS
node smoke.mjs
cat >consumer.mts <<'TS'
import { create } from '@bufbuild/protobuf';
import { WebSocketRequestSchema } from '@rimelabs/api';
import type { WebSocketRequest } from '@rimelabs/api';
const request: WebSocketRequest = create(WebSocketRequestSchema, {
  contextId: 'test', payload: { case: 'text', value: 'Hello.' },
});
// @ts-expect-error The generated type must reject non-string text.
const invalid: WebSocketRequest = { ...request, payload: { case: 'text', value: 42 } };
TS
cp consumer.mts consumer.cts
./node_modules/.bin/tsc --strict --noEmit --target es2022 --module nodenext \
	consumer.mts consumer.cts
echo 'Installed npm package passes ESM, CommonJS, and TypeScript checks.'
