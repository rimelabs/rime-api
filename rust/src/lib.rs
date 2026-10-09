//! Rime protocol types generated from the canonical Protobuf definitions.
//!
//! The default `grpc` feature includes Tonic clients and servers. Disable it
//! when only message encoding is needed. Serde uses the Protobuf JSON mapping.
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// Google RPC status messages referenced by the speech protocol.
pub mod google {
    /// Structured RPC status.
    // Generator output can trigger new Clippy lints on newer toolchains.
    #[allow(clippy::all)]
    pub mod rpc {
        include!("generated/google.rpc.rs");
        include!("generated/google.rpc.serde.rs");
    }
}

/// Text-to-speech and speech-to-text messages and gRPC services.
// Keep style lints on handwritten code, not on pinned generator output.
#[allow(clippy::all)]
pub mod rime {
    include!("generated/rime.rs");
    include!("generated/rime.serde.rs");
}

pub use rime::*;
