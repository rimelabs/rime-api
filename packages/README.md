# Rime API

Message definitions and optional gRPC clients for Rime text-to-speech and speech-to-text. Python and JavaScript
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
Applications and framework adapters supply endpoints, TLS settings, authentication
metadata, cancellation policy, and audio I/O. The package root remains usable
without a gRPC runtime. Native gRPC clients are available through the imports below.

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

## gRPC clients

Install the Python extra to use the standard generated client stubs with
`grpcio>=1.71.2,<2`. Both synchronous and `grpc.aio` channels are supported.

```shell
uv add 'rime-api[grpc]'
```

The modules contain clients for both services. Pass an existing channel to the
client for the service hosted by your target:

```python
from rime_api.speech_to_text_pb2_grpc import SpeechToTextStub
from rime_api.text_to_speech_pb2_grpc import TextToSpeechStub

# For a speech-to-text channel:
recognition = SpeechToTextStub(channel)
# For a text-to-speech channel:
synthesis = TextToSpeechStub(channel)
```

Node.js uses the separate `@rimelabs/api/grpc` entry point. Install its optional
peer dependency, `@grpc/grpc-js`, version 1.14.0 or newer within major version 1.

```shell
npm install @rimelabs/api @grpc/grpc-js@^1.14.0 @bufbuild/protobuf@^2.11.0
```

These constructors accept the same connection settings as a native grpc-js
client. Replace each target with the address of its service:

```typescript
import { credentials } from "@grpc/grpc-js";
import { SpeechToTextClient, TextToSpeechClient } from "@rimelabs/api/grpc";

const recognition = new SpeechToTextClient("stt-target:443", credentials.createSsl());
const synthesis = new TextToSpeechClient("tts-target:443", credentials.createSsl());
// Close clients when the application no longer needs their connections.
recognition.close();
synthesis.close();
```

Use the existing package message schemas with `create` from `@bufbuild/protobuf`.
The methods use lower camel case: `transcribe`, `transcribeStreaming`, `synthesize`,
`synthesizeStreaming`, `normalizeText`, `getSupportedLanguages`, and
`getSupportedSpeakers`. Unary calls accept a callback. Streaming calls return
native grpc-js streams. Set metadata and deadlines with the standard call options.
ESM, CommonJS, and TypeScript imports are supported. The gRPC entry point is for
Node.js, while the message exports remain usable in browsers.

## License

Copyright 2026 Rime. Licensed under the Apache License, Version 2.0.
See `LICENSE` and `NOTICE` included in each distribution.

This license covers the public Protobuf and AsyncAPI definitions and the
Rime-owned generated code and documentation in these distributions. It does
not apply to other Rime monorepo components. Dependencies retain their own
licenses.
