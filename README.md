# Rime API

Public API definitions and generated Python and JavaScript packages for Rime
text-to-speech.

| Definition | Purpose |
| --- | --- |
| [Protobuf](schema/rime/text_to_speech.proto) | Synthesis, text normalization, language and speaker queries, and WebSocket messages |
| [AsyncAPI](schema/text_to_speech.asyncapi.yaml) | WebSocket framing, authentication, and connection behavior |

`rimelabs/rime` owns the source definitions. Copybara copies the two public
files into this repository. This repository owns package generation, tests,
versioning, and publication. See [schema sync](sync/README.md).

## Packages

The first release is prepared as `0.0.1`. The packages are not yet published.
After publication, install them with:

```shell
uv add rime-api
npm install @rimelabs/api @bufbuild/protobuf@^2.11.0
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

Python requires version 3.10 or newer. JavaScript supports ESM and CommonJS,
with TypeScript declarations. These packages supply message definitions.
Applications and framework adapters manage connections, authentication,
cancellation, and audio playback. See [package details](packages/README.md).

## Build and test

Install Bazelisk, then run these commands from this repository:

```shell
bazel build //:packages
bazel test //tests/...
bash tools/check_compatibility.sh
bazel run //:format
```

Bazel pins Python, Node.js, the generators, Buf, and Copybara. No GPU, CUDA,
private cache, or access to the source repository is needed to build or test.
The output archives are in `bazel-bin/packages/dist/`.

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

See [release instructions](RELEASING.md) to publish tested archives.

## License

Copyright 2026 Rime. This repository uses the [Apache License 2.0](LICENSE).
Dependencies retain their own licenses. This license does not cover other
components in the Rime monorepo.
