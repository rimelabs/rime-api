# Rime API for Go

Generated TTS and STT message types and gRPC clients. This module contains protocol
definitions. Use `github.com/rimelabs/rime-sdk/go` for the TTS SDK.

```sh
go get github.com/rimelabs/rime-api/go@latest
```

```go
import (
    api "github.com/rimelabs/rime-api/go"
    "google.golang.org/protobuf/proto"
)

request := &api.SynthesisRequest{Text: "Hello."}
encoded, err := proto.Marshal(request)
```

Go 1.24 or later is required. The package includes `NewTextToSpeechClient` and
`NewSpeechToTextClient`. Applications supply the connection and credentials.
Installation requires no protobuf compiler or access to the private source.

The canonical schemas remain in `rimelabs/rime`. Copybara exports them into
`rime-api`, where Bazel generates this module. Do not edit `.pb.go`, `schema/`,
`SOURCE.json`, licenses, or shared test fixtures here. From the repository root:

```sh
bazel run //:update_go
bazel run //:update_go -- --check
```

The generated files are committed so Go can install a tested Git tag. Schema
sync updates them in the same PR; CI rejects stale files. `SOURCE.json` records
schema hashes. Module versions use `go/vX.Y.Z` tags and share `VERSION` with the
Python and TypeScript packages. Release CI verifies the public Go proxy download
before completing the GitHub release. Go needs no separate registry account.

The module uses Apache-2.0. See `LICENSE`, `NOTICE`, and `PROTOBUF_LICENSE`.
