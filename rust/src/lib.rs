//! Rime protocol types generated from the canonical Protobuf definitions.
//!
//! The default `grpc` feature includes Tonic clients and servers. Disable it
//! when only message encoding is needed. Serde uses the Protobuf JSON mapping.
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// Google RPC status messages referenced by the speech protocol.
pub mod google {
    /// Structured RPC status.
    pub mod rpc {
        include!("generated/google.rpc.rs");
        include!("generated/google.rpc.serde.rs");
    }
}

/// Text-to-speech and speech-to-text messages and gRPC services.
pub mod rime {
    include!("generated/rime.rs");
    include!("generated/rime.serde.rs");
}

pub use rime::*;
