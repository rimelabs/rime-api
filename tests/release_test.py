"""Check release source validation and recovery from partial publication."""

import base64
import copy
import hashlib
import io
import json
from pathlib import Path
import sys
import tarfile
import unittest
from unittest.mock import patch
import zipfile

from tools import release


PACKAGES = Path(sys.argv.pop(1))


def rewrite_archive(name, contents, transform):
    output = io.BytesIO()
    if name.endswith(".whl"):
        with (
            zipfile.ZipFile(io.BytesIO(contents)) as source,
            zipfile.ZipFile(output, "w") as destination,
        ):
            for member in source.infolist():
                data = transform(member.filename, source.read(member))
                if data is not None:
                    destination.writestr(member, data)
    else:
        with (
            tarfile.open(fileobj=io.BytesIO(contents)) as source,
            tarfile.open(fileobj=output, mode="w:gz") as destination,
        ):
            for member in source.getmembers():
                if not member.isfile():
                    destination.addfile(member)
                    continue
                data = transform(member.name, source.extractfile(member).read())
                if data is not None:
                    member.size = len(data)
                    destination.addfile(member, io.BytesIO(data))
    return output.getvalue()


class ReleaseTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.files = {
            path.name: path.read_bytes() for path in (PACKAGES / "dist").iterdir()
        }
        source = json.loads(cls.files["SOURCE.json"])
        cls.version = source["version"]
        cls.schemas = {
            name: (PACKAGES / "javascript/schema" / name).read_bytes()
            for name in source["schemas"]
        }

    def test_accepts_built_archives(self):
        release.validate_archives(self.files, self.version, self.schemas)

    def test_rust_archive_is_required_and_validated_when_present_in_commit(self):
        files = dict(self.files)
        with self.assertRaisesRegex(ValueError, "Unexpected artifact files"):
            release.validate_archives(files, self.version, self.schemas, rust=True)
        output = io.BytesIO()
        contents = {
            "Cargo.toml": f'[package]\nname = "rime-api"\nversion = "{self.version}"\nlicense = "Apache-2.0"\n'.encode(),
            "SOURCE.json": files["SOURCE.json"],
        }
        for name in (
            "rime.rs",
            "rime.serde.rs",
            "google.rpc.rs",
            "google.rpc.serde.rs",
        ):
            contents[f"src/generated/{name}"] = b"// generated\n"
        with tarfile.open(fileobj=output, mode="w:gz") as archive:
            for name, data in contents.items():
                member = tarfile.TarInfo(f"rime-api-{self.version}/{name}")
                member.size = len(data)
                archive.addfile(member, io.BytesIO(data))
        name = release.archive_names(self.version, rust=True)["crates"][0]
        files[name] = output.getvalue()
        release.validate_archives(files, self.version, self.schemas, rust=True)
        files[name] = rewrite_archive(
            name,
            files[name],
            lambda path, data: (
                b'{"version":"wrong"}' if path.endswith("SOURCE.json") else data
            ),
        )
        with self.assertRaisesRegex(ValueError, "Rust source record"):
            release.validate_archives(files, self.version, self.schemas, rust=True)

    def test_rejects_wrong_source_commit(self):
        for name in self.schemas:
            with self.subTest(schema=name):
                schemas = dict(self.schemas, **{name: self.schemas[name] + b"\n"})
                with self.assertRaisesRegex(ValueError, "selected commit"):
                    release.validate_archives(self.files, self.version, schemas)

    def test_rejects_extra_artifact_files(self):
        with self.assertRaisesRegex(ValueError, "Unexpected artifact files"):
            release.validate_archives(
                dict(self.files, unexpected=b""), self.version, self.schemas
            )

    def test_rejects_source_record_without_stt(self):
        files = dict(self.files)
        source = json.loads(files["SOURCE.json"])
        del source["schemas"]["rime/speech_to_text.proto"]
        files["SOURCE.json"] = json.dumps(source).encode()
        with self.assertRaisesRegex(ValueError, "selected commit"):
            release.validate_archives(files, self.version, self.schemas)

    def test_rejects_changed_stt_in_each_archive(self):
        for name, contents in self.files.items():
            if name == "SOURCE.json":
                continue
            for schema in ("speech_to_text.proto", "speech_to_text.asyncapi.yaml"):
                with self.subTest(archive=name, schema=schema):
                    changed = rewrite_archive(
                        name,
                        contents,
                        lambda path, data: (
                            data + b"\n" if path.endswith("/" + schema) else data
                        ),
                    )
                    with self.assertRaisesRegex(ValueError, "Archive schema differs"):
                        release.validate_archives(
                            dict(self.files, **{name: changed}),
                            self.version,
                            self.schemas,
                        )

    def test_legacy_source_record_can_recover_only_tts_commits(self):
        schemas = {
            name: data
            for name, data in self.schemas.items()
            if "speech_to_text" not in name
        }
        source = json.dumps(
            {
                "version": self.version,
                "schema": "rime/text_to_speech.proto",
                "sha256": hashlib.sha256(
                    schemas["rime/text_to_speech.proto"]
                ).hexdigest(),
                "asyncapi": {
                    "schema": "text_to_speech.asyncapi.yaml",
                    "sha256": hashlib.sha256(
                        schemas["text_to_speech.asyncapi.yaml"]
                    ).hexdigest(),
                },
            }
        ).encode()

        def legacy(path, data):
            if "speech_to_text" in path:
                return None
            return source if path.endswith("/SOURCE.json") else data

        files = {
            name: source
            if name == "SOURCE.json"
            else rewrite_archive(name, contents, legacy)
            for name, contents in self.files.items()
        }
        release.validate_archives(files, self.version, schemas)
        with self.assertRaisesRegex(ValueError, "selected commit"):
            release.validate_archives(files, self.version, self.schemas)

    def test_rejects_mismatched_archive_version(self):
        files = dict(self.files)
        names = release.archive_names(self.version)
        replacement = release.archive_names("99.0.0")
        for registry in names:
            for original, renamed in zip(names[registry], replacement[registry]):
                files[renamed] = files.pop(original)
        source = json.loads(files["SOURCE.json"])
        source["version"] = "99.0.0"
        files["SOURCE.json"] = json.dumps(source).encode()
        with self.assertRaisesRegex(ValueError, "npm metadata differs"):
            release.validate_archives(files, "99.0.0", self.schemas)

    def test_requires_successful_main_run(self):
        run = {
            "id": 123,
            "repository": {"full_name": release.REPOSITORY},
            "head_repository": {"full_name": release.REPOSITORY},
            "head_branch": "main",
            "event": "push",
            "status": "completed",
            "conclusion": "success",
            "path": ".github/workflows/check.yaml",
            "head_sha": "a" * 40,
        }
        release.validate_run(run, 123)
        for key, value in {
            "id": 124,
            "head_repository": {"full_name": "fork/rime-api"},
            "head_branch": "feature",
            "event": "pull_request",
            "status": "in_progress",
            "conclusion": "failure",
            "path": ".github/workflows/release.yaml",
            "head_sha": "invalid",
        }.items():
            with self.subTest(field=key), self.assertRaises(ValueError):
                release.validate_run(dict(run, **{key: value}), 123)

    def test_npm_retry_requires_identical_archive(self):
        name = release.archive_names(self.version)["npm"][0]
        metadata = {
            "name": "@rimelabs/api",
            "version": self.version,
            "dist": {
                "integrity": "sha512-"
                + base64.b64encode(hashlib.sha512(self.files[name]).digest()).decode(),
                "tarball": "https://registry.npmjs.org/archive.tgz",
            },
        }
        self.assertEqual(
            release.compare_registry("npm", metadata, self.files, self.version),
            {name: metadata["dist"]["tarball"]},
        )
        metadata["dist"]["integrity"] = "sha512-different"
        with self.assertRaisesRegex(ValueError, "different bytes"):
            release.compare_registry("npm", metadata, self.files, self.version)

    def test_partial_pypi_release_accepts_only_matching_files(self):
        name = release.archive_names(self.version)["pypi"][0]
        metadata = {
            "info": {"version": self.version},
            "urls": [
                {
                    "filename": name,
                    "digests": {"sha256": hashlib.sha256(self.files[name]).hexdigest()},
                    "url": "https://files.pythonhosted.org/archive.whl",
                }
            ],
        }
        self.assertEqual(
            len(release.compare_registry("pypi", metadata, self.files, self.version)), 1
        )
        for field, value in [
            ("filename", "unexpected.whl"),
            ("digests", {"sha256": "different"}),
        ]:
            invalid = copy.deepcopy(metadata)
            invalid["urls"][0][field] = value
            with self.subTest(field=field), self.assertRaises(ValueError):
                release.compare_registry("pypi", invalid, self.files, self.version)

    def test_missing_registry_version_is_publishable(self):
        for registry in ["npm", "pypi"]:
            self.assertEqual(
                release.compare_registry(registry, None, self.files, self.version), {}
            )

    def test_registry_errors_do_not_mean_package_is_missing(self):
        for status in [401, 403, 429, 500]:
            error = release.HTTPError("https://example.com", status, "test", {}, None)
            with (
                patch.object(release, "fetch", side_effect=error),
                self.assertRaises(release.HTTPError),
            ):
                release.registry_metadata("npm", self.version)


if __name__ == "__main__":
    unittest.main()
