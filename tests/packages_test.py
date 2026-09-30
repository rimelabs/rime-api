"""Check Python imports and the shared public binary/JSON contract."""

import argparse
import hashlib
from importlib import metadata, resources
import json
from pathlib import Path
import pickle
import sys
import unittest
import zipfile

from google.protobuf import json_format


class PackageTest(unittest.TestCase):
    def test_package_metadata(self):
        distribution = metadata.metadata("rime-api")
        self.assertEqual(distribution["Requires-Python"], ">=3.10")
        dependencies = distribution.get_all("Requires-Dist")
        self.assertCountEqual(
            dependencies, ["protobuf>=5.29.6", "googleapis-common-protos>=1.70.0"]
        )
        source = json.loads(
            resources.files("rime_api").joinpath("SOURCE.json").read_text()
        )
        self.assertEqual(source["version"], distribution["Version"])
        package = resources.files("rime_api")
        if packages is not None or "schemas" in source:
            self.assertEqual(
                set(source["schemas"]),
                {
                    "rime/text_to_speech.proto",
                    "text_to_speech.asyncapi.yaml",
                    "rime/speech_to_text.proto",
                    "speech_to_text.asyncapi.yaml",
                },
            )
            hashes = source["schemas"]
        else:
            # Registry recovery can install an older TTS-only release.
            hashes = {
                source["schema"]: source["sha256"],
                source["asyncapi"]["schema"]: source["asyncapi"]["sha256"],
            }
        for name, digest in hashes.items():
            with self.subTest(schema=name):
                self.assertEqual(
                    digest,
                    hashlib.sha256(
                        package.joinpath("schema/" + name).read_bytes()
                    ).hexdigest(),
                )

    def test_shared_fixtures(self):
        source = (
            resources.files("rime_api")
            .joinpath("schema/rime/text_to_speech.proto")
            .read_text()
        )
        for fixture in fixtures:
            with self.subTest(fixture=fixture):
                # A support PR also builds the older, unsynced schema. Select
                # new fixtures from the source, never from generated exports.
                if (
                    fixture.get("requires")
                    and f"message {fixture['requires']} {{" not in source
                ):
                    continue
                definitions = speech_proto if fixture.get("api") == "stt" else proto
                if definitions is None:
                    continue
                message_type = getattr(definitions, fixture["message"])
                message = json_format.ParseDict(fixture["json"], message_type())
                self.assertEqual(message.SerializeToString().hex(), fixture["hex"])
                decoded = message_type.FromString(bytes.fromhex(fixture["hex"]))
                self.assertEqual(json_format.MessageToDict(decoded), fixture["json"])

    def test_namespace_and_pickle(self):
        message = proto.WebSocketRequest(text="hello")
        self.assertEqual(message.__class__.__module__, "rime_api.text_to_speech_pb2")
        self.assertEqual(pickle.loads(pickle.dumps(message)), message)
        self.assertEqual(proto.DESCRIPTOR.package, "rime")
        self.assertNotIn("rime", sys.modules)

    def test_complete_public_schema(self):
        service = proto.DESCRIPTOR.services_by_name["TextToSpeech"]
        self.assertEqual(service.full_name, "rime.TextToSpeech")
        self.assertTrue(service.methods_by_name["SynthesizeStreaming"].client_streaming)
        self.assertEqual(
            service.methods_by_name["Synthesize"].input_type,
            proto.SynthesisRequest.DESCRIPTOR,
        )

    def test_optional_presence_and_oneof(self):
        parameters = proto.AudioParameters()
        self.assertFalse(parameters.HasField("sampling_rate"))
        parameters.sampling_rate = 0
        self.assertTrue(parameters.HasField("sampling_rate"))
        request = proto.WebSocketRequest(text="hello")
        request.end.SetInParent()
        self.assertEqual(request.WhichOneof("payload"), "end")
        self.assertEqual(request.text, "")

    def test_speech_to_text_schema(self):
        if speech_proto is None:
            self.skipTest("Older TTS-only release")
        service = speech_proto.DESCRIPTOR.services_by_name["SpeechToText"]
        self.assertEqual(service.full_name, "rime.SpeechToText")
        self.assertEqual(
            service.methods_by_name["Transcribe"].input_type,
            speech_proto.TranscriptionRequest.DESCRIPTOR,
        )
        streaming = service.methods_by_name["TranscribeStreaming"]
        self.assertTrue(streaming.client_streaming)
        self.assertTrue(streaming.server_streaming)
        request = speech_proto.SpeechWebSocketRequest(audio=b"\x01\x02")
        self.assertEqual(request.__class__.__module__, "rime_api.speech_to_text_pb2")
        self.assertEqual(pickle.loads(pickle.dumps(request)), request)
        request.end.SetInParent()
        self.assertEqual(request.WhichOneof("payload"), "end")
        self.assertEqual(request.audio, b"")
        config = speech_proto.StreamingConfig()
        self.assertFalse(config.HasField("language"))
        config.language = ""
        self.assertTrue(config.HasField("language"))

    def test_unknown_binary_fields(self):
        wire = bytes.fromhex("220568656c6c6ff80701")
        message = proto.WebSocketRequest.FromString(wire)
        self.assertEqual(message.text, "hello")
        self.assertEqual(message.SerializeToString(), wire)

    def test_typing_files(self):
        if packages is None:
            directory = Path(proto.__file__).parent
            self.assertTrue((directory / "py.typed").is_file())
            self.assertTrue((directory / "text_to_speech_pb2.pyi").is_file())
            if speech_proto is not None:
                self.assertTrue((directory / "speech_to_text_pb2.pyi").is_file())
        else:
            with zipfile.ZipFile(next((packages / "dist").glob("*.whl"))) as wheel:
                self.assertIn("rime_api/py.typed", wheel.namelist())
                self.assertIn("rime_api/text_to_speech_pb2.pyi", wheel.namelist())
                self.assertIn("rime_api/speech_to_text_pb2.pyi", wheel.namelist())
                self.assertFalse(
                    any(name.startswith("rime/") for name in wheel.namelist())
                )


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--packages", type=Path)
    parser.add_argument("--fixtures", type=Path, required=True)
    arguments = parser.parse_args()
    packages = arguments.packages
    if packages:
        sys.path.insert(0, str(next((packages / "dist").glob("*.whl"))))
    from rime_api import text_to_speech_pb2 as proto

    source_record = json.loads(
        resources.files("rime_api").joinpath("SOURCE.json").read_text()
    )
    speech_proto = None
    if packages is not None or "schemas" in source_record:
        from rime_api import speech_to_text_pb2 as speech_proto

    fixtures = json.loads(arguments.fixtures.read_text())
    unittest.main(argv=[sys.argv[0]])
