use prost::Message;
use rimelabs_api::*;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

fn check<M: Message + Default + DeserializeOwned + Serialize + PartialEq + std::fmt::Debug>(
    fixture: &Value,
) {
    let message: M = serde_json::from_value(fixture["json"].clone()).unwrap();
    let encoded = message.encode_to_vec();
    let hex: String = encoded.iter().map(|byte| format!("{byte:02x}")).collect();
    assert_eq!(hex, fixture["hex"].as_str().unwrap());
    let decoded = M::decode(encoded.as_slice()).unwrap();
    assert_eq!(decoded, message);
    assert_eq!(serde_json::to_value(decoded).unwrap(), fixture["json"]);
}

#[test]
fn shared_binary_and_protobuf_json_fixtures() {
    let fixtures = option_env!("RIME_API_FIXTURES")
        .map(str::to_owned)
        .unwrap_or_else(|| format!("{}/../tests/fixtures.json", env!("CARGO_MANIFEST_DIR")));
    let fixtures: Value = serde_json::from_slice(&std::fs::read(fixtures).unwrap()).unwrap();
    for fixture in fixtures.as_array().unwrap() {
        match fixture["message"].as_str().unwrap() {
            "WebSocketRequest" => check::<WebSocketRequest>(fixture),
            "WebSocketResponse" => check::<WebSocketResponse>(fixture),
            "SpanTimestamp" => check::<SpanTimestamp>(fixture),
            "Timestamps" => check::<Timestamps>(fixture),
            "SynthesisResponseStream" => check::<SynthesisResponseStream>(fixture),
            "TranscriptionRequest" => check::<TranscriptionRequest>(fixture),
            "SpeechWebSocketRequest" => check::<SpeechWebSocketRequest>(fixture),
            "SpeechWebSocketResponse" => check::<SpeechWebSocketResponse>(fixture),
            "StreamingTranscriptionRequest" => check::<StreamingTranscriptionRequest>(fixture),
            name => panic!("missing Rust fixture mapping for {name}"),
        }
    }
}
