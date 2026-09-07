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
        self.assertEqual(len(dependencies), 1)
        self.assertTrue(dependencies[0].startswith("protobuf"))
        source = json.loads(
            resources.files("rime_api").joinpath("SOURCE.json").read_text()
        )
        self.assertEqual(source["version"], distribution["Version"])
        self.assertEqual(source["schema"], "rime/text_to_speech.proto")
        self.assertEqual(len(source["sha256"]), 64)
        package = resources.files("rime_api")
        schema = package.joinpath("schema/rime/text_to_speech.proto").read_bytes()
        asyncapi = package.joinpath("schema/text_to_speech.asyncapi.yaml").read_bytes()
        self.assertEqual(source["sha256"], hashlib.sha256(schema).hexdigest())
        self.assertEqual(
            source["asyncapi"]["sha256"], hashlib.sha256(asyncapi).hexdigest()
        )

    def test_shared_fixtures(self):
        for fixture in fixtures:
            with self.subTest(fixture=fixture):
                message_type = getattr(proto, fixture["message"])
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
            proto.SynthesisRequest.DESCRIPTOR.fields_by_name[
                "arcana_parameters"
            ].number,
            7,
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
        else:
            with zipfile.ZipFile(next((packages / "dist").glob("*.whl"))) as wheel:
                self.assertIn("rime_api/py.typed", wheel.namelist())
                self.assertIn("rime_api/text_to_speech_pb2.pyi", wheel.namelist())
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

    fixtures = json.loads(arguments.fixtures.read_text())
    unittest.main(argv=[sys.argv[0]])
