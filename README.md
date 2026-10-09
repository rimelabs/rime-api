# Rime API

Public API definitions and generated Python, JavaScript, Go, and Rust packages for Rime
text-to-speech and speech-to-text.

| Definition | Purpose |
| --- | --- |
| [Protobuf](schema/rime/text_to_speech.proto) | Synthesis, text normalization, language and speaker queries, and WebSocket messages |
| [AsyncAPI](schema/text_to_speech.asyncapi.yaml) | WebSocket framing, authentication, and connection behavior |
| [STT Protobuf](schema/rime/speech_to_text.proto) | Transcription requests, results, and streaming WebSocket messages |
| [STT AsyncAPI](schema/speech_to_text.asyncapi.yaml) | Recognition WebSocket framing and connection behavior |

`rimelabs/rime` owns the source definitions. Copybara copies the four public
files into this repository. This repository owns package generation, tests,
versioning, and publication. See [schema sync](sync/README.md).

## Packages

Python [`rime-api`](https://pypi.org/project/rime-api/) and npm
[`@rimelabs/api`](https://www.npmjs.com/package/@rimelabs/api) share the version
in `VERSION` with the [Go module](go/README.md). Install them with:

```shell
uv add rime-api
npm install @rimelabs/api @bufbuild/protobuf@^2.11.0
go get github.com/rimelabs/rime-api/go@latest
cargo add rime-api
```

```python
from rime_api import text_to_speech_pb2 as proto

request = proto.WebSocketRequest(context_id='turn-42', text='Hello.')
encoded = request.SerializeToString()
```

```typescript
import { create, toBinary } from "@bufbuild/protobuf";
import { WebSocketRequestSchema } from "@rimelabs/api";

const request = create(WebSocketRequestSchema, {
  contextId: "turn-42",
  payload: { case: "text", value: "Hello." },
});
const encoded = toBinary(WebSocketRequestSchema, request);
```

For speech-to-text, use `from rime_api import speech_to_text_pb2` in Python
or import `SpeechWebSocketRequestSchema` from `@rimelabs/api` in JavaScript.
See [STT examples](packages/README.md#speech-to-text).

Python requires version 3.10 or newer. JavaScript supports ESM and CommonJS,
with TypeScript declarations. These packages supply message definitions.
Applications and framework adapters manage connections, authentication,
cancellation, and audio playback. See [package details](packages/README.md).

## Build and test

Install Bazelisk, Git, and jq, then run these commands from this repository:

```shell
bazel build //:packages
bazel run //:update_go -- --check
bazel run //:update_rust -- --check
bazel test //tests/...
bash tools/check_compatibility.sh
bazel run //:format
```

Bazel pins Python, Node.js, Go, the generators, Buf, and Copybara. No GPU, CUDA,
private cache, or access to the source repository is needed to build or test.
The output archives are in `bazel-bin/packages/dist/`.

The [Rust crate](rust/README.md) contains generated TTS and STT messages with
Protobuf JSON support. Its default `grpc` feature includes Tonic clients and
servers. It requires Rust 1.88 or newer. Use `bazel run //:cargo -- test -p rime-api`
for Cargo checks and `bazel run //:cargo -- package -p rime-api --locked` for the
crate archive in `target/package/`. Consumers do not need Bazel or protoc.

The tests cover shared binary and JSON fixtures, Python imports and typing,
JavaScript module formats, and Copybara export behavior. CI also installs the
archives across Python 3.10 through 3.14 and Node.js 20, 22, and 24.

Python dependency changes use uv:

```shell
uv lock
uv export --locked --format requirements-txt --no-emit-project --output-file requirements.txt
```

Node.js dependency changes use the pinned pnpm version in `package.json`.
Commit both dependency manifests and their lockfiles.

Release Please prepares a shared version and changelog in a release PR.
Merging that PR starts publication after package checks pass on the merged
commit. See [release instructions](RELEASING.md) for setup and recovery.

## License

Copyright 2026 Rime. This repository uses the [Apache License 2.0](LICENSE).
Dependencies retain their own licenses. This license does not cover other
components in the Rime monorepo.
