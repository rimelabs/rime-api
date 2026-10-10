# Rime API for Rust

Generated TTS and STT Protobuf types, Protobuf JSON support, and Tonic clients
and servers. The default `grpc` feature enables gRPC. Disable default features
to use messages alone. Requires Rust 1.88 or later.

```rust
use prost::Message;
use rimelabs_api::SynthesisRequest;

let request = SynthesisRequest {
    text: "Hello.".into(),
    ..Default::default()
};
let bytes = request.encode_to_vec();
assert_eq!(SynthesisRequest::decode(bytes.as_slice()).unwrap(), request);
```

These types do not manage authentication, audio conversion, or stream lifetime.
Use the Rime SDK for those operations.

Maintainers generate sources with `bazel run //:update_rust` and check them with
`bazel run //:update_rust -- --check`. Do not edit `src/generated` by hand.
The published crate contains generated sources; consumers need only Cargo.
