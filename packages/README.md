# Rime API

Generated message definitions for Rime text-to-speech and speech-to-text. Python and JavaScript
packages use the same schema and release version.

## Python

```shell
uv add rime-api
```

```python
from rime_api import text_to_speech_pb2 as proto

request = proto.WebSocketRequest(context_id='turn-42', text='Hello.')
encoded = request.SerializeToString()
```

Python 3.10 or newer is required. The package includes type stubs. Runtime
dependencies are `protobuf>=5.29.6` and `googleapis-common-protos>=1.70.0`.
It does not require gRPC.

## JavaScript and TypeScript

```shell
npm install @rimelabs/api @bufbuild/protobuf@^2.11.0
```

```typescript
import { create, toBinary } from "@bufbuild/protobuf";
import { WebSocketRequestSchema } from "@rimelabs/api";
import type { WebSocketRequest } from "@rimelabs/api";

const request: WebSocketRequest = create(WebSocketRequestSchema, {
  contextId: "turn-42",
  payload: { case: "text", value: "Hello." },
});
const encoded = toBinary(WebSocketRequestSchema, request);
```

The package supports ESM and CommonJS and includes TypeScript declarations.
Consumers that import `@bufbuild/protobuf` directly should declare it as a
direct dependency too. Use its `fromJson` and `toJson` functions for the Rime
JSON protocol. `JSON.stringify` does not implement the protobuf JSON mapping.

## Speech-to-text

Python uses a separate module for STT:

```python
from rime_api import speech_to_text_pb2 as proto

request = proto.SpeechWebSocketRequest(
    start=proto.StreamingConfig(
        output_contract=proto.STREAMING_OUTPUT_CONTRACT_REVISED_HYPOTHESES,
        language="en",
    )
)
encoded = request.SerializeToString()
```

JavaScript and TypeScript export STT definitions from the same package:

```typescript
import { create, toBinary } from "@bufbuild/protobuf";
import {
  SpeechWebSocketRequestSchema,
  StreamingOutputContract,
} from "@rimelabs/api";

const request = create(SpeechWebSocketRequestSchema, {
  payload: {
    case: "start",
    value: {
      outputContract: StreamingOutputContract.REVISED_HYPOTHESES,
      language: "en",
    },
  },
});
const encoded = toBinary(SpeechWebSocketRequestSchema, request);
```

The STT schema includes whole-utterance transcription, bidirectional streaming,
and WebSocket messages. See `schema/rime/speech_to_text.proto` and
`schema/speech_to_text.asyncapi.yaml` in the package for the full contract.

## Scope and compatibility

These packages contain the full public TTS and STT schemas, including synthesis,
normalization, language and speaker queries, transcription, and WebSocket envelopes.
Framework adapters own connections, authentication, cancellation, and audio I/O.
Service descriptors are included; gRPC client stubs are not included.

The protobuf package remains `rime`. The Python generator uses a virtual source
paths, `rime_api/text_to_speech.proto` and `rime_api/speech_to_text.proto`,
to obtain the Python import names. This changes the Python descriptor file names, but preserves message names,
service names, field numbers, and both wire formats. Do not load independently
generated definitions of these same `rime` messages in one Python process.

The Python wheel, source archive, and npm archive include the exact Protobuf
and AsyncAPI definitions. The `schemas` map in `SOURCE.json` records each
public definition's path and SHA-256 hash. In the
wheel, the definitions are under `rime_api/schema/`. The other archives use
`schema/`. Generated code is part of the distribution;
installation requires no protobuf compiler or access to the Rime repository.

## License

Copyright 2026 Rime. Licensed under the Apache License, Version 2.0.
See `LICENSE` and `NOTICE` included in each distribution.

This license covers the public Protobuf and AsyncAPI definitions and the
Rime-owned generated code and documentation in these distributions. It does
not apply to other Rime monorepo components. Dependencies retain their own
licenses.
