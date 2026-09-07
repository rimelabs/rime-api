"""Validate tested release archives and compare them with public registries."""

import argparse
import base64
from email.parser import BytesParser
import hashlib
import io
import json
from pathlib import Path
import re
import subprocess
import tarfile
import time
from urllib.error import HTTPError
from urllib.parse import urlparse
from urllib.request import urlopen
import zipfile


REPOSITORY = "rimelabs/rime-api"


def require(condition, message):
    if not condition:
        raise ValueError(message)


def command(*arguments):
    return subprocess.run(arguments, check=True, stdout=subprocess.PIPE).stdout


def github(path):
    return json.loads(command("gh", "api", f"repos/{REPOSITORY}/{path}"))


def github_pages(path, key):
    pages = json.loads(
        command("gh", "api", "--paginate", "--slurp", f"repos/{REPOSITORY}/{path}")
    )
    return [item for page in pages for item in page[key]]


def validate_run(run, run_id):
    require(run["id"] == run_id, "The workflow run ID does not match")
    require(run["repository"]["full_name"] == REPOSITORY, "Unexpected repository")
    require(
        run["head_repository"]["full_name"] == REPOSITORY, "Fork runs cannot release"
    )
    require(run["head_branch"] == "main", "The test run must be from main")
    require(
        run["event"] in {"push", "workflow_dispatch"}, "PR and tag runs cannot release"
    )
    require(run["path"] == ".github/workflows/check.yaml", "Select Check packages")
    require(
        run["status"] == "completed" and run["conclusion"] == "success",
        "The test run must have completed successfully",
    )
    require(re.fullmatch(r"[0-9a-f]{40}", run["head_sha"]), "Invalid source commit")


def archive_names(version):
    require(
        re.fullmatch(r"(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)", version),
        "Version must be major.minor.patch",
    )
    return {
        "npm": [f"rime-api-{version}.tgz"],
        "pypi": [f"rime_api-{version}-py3-none-any.whl", f"rime_api-{version}.tar.gz"],
    }


def validate_archives(files, version, schemas):
    names = archive_names(version)
    require(
        set(files) == {"SOURCE.json", *names["npm"], *names["pypi"]},
        "Unexpected artifact files",
    )
    source = json.loads(files["SOURCE.json"])
    expected_source = {
        "version": version,
        "schema": "rime/text_to_speech.proto",
        "sha256": hashlib.sha256(schemas["rime/text_to_speech.proto"]).hexdigest(),
        "asyncapi": {
            "schema": "text_to_speech.asyncapi.yaml",
            "sha256": hashlib.sha256(
                schemas["text_to_speech.asyncapi.yaml"]
            ).hexdigest(),
        },
    }
    require(source == expected_source, "SOURCE.json does not match the selected commit")

    def check_contents(read, prefix):
        require(
            json.loads(read(prefix + "SOURCE.json")) == source,
            "Archive source record differs",
        )
        for name, contents in schemas.items():
            require(
                read(prefix + "schema/" + name) == contents, "Archive schema differs"
            )

    def check_python(metadata):
        parsed = BytesParser().parsebytes(metadata)
        require(
            parsed["Name"] == "rime-api" and parsed["Version"] == version,
            "Python metadata differs",
        )

    with tarfile.open(fileobj=io.BytesIO(files[names["npm"][0]])) as archive:

        def read(name):
            return archive.extractfile(name).read()

        package = json.loads(read("package/package.json"))
        require(
            package["name"] == "@rimelabs/api" and package["version"] == version,
            "npm metadata differs",
        )
        require(
            package["publishConfig"]
            == {"access": "public", "registry": "https://registry.npmjs.org"},
            "Unexpected npm publication settings",
        )
        check_contents(read, "package/")
    with zipfile.ZipFile(io.BytesIO(files[names["pypi"][0]])) as archive:
        check_python(archive.read(f"rime_api-{version}.dist-info/METADATA"))
        check_contents(archive.read, "rime_api/")
    with tarfile.open(fileobj=io.BytesIO(files[names["pypi"][1]])) as archive:

        def read(name):
            return archive.extractfile(name).read()

        check_python(read(f"rime_api-{version}/PKG-INFO"))
        check_contents(read, f"rime_api-{version}/")


def prepare(run_id, version, directory):
    archive_names(version)
    run = github(f"actions/runs/{run_id}")
    validate_run(run, run_id)
    commit = run["head_sha"]
    command("git", "merge-base", "--is-ancestor", commit, "HEAD")
    require(
        command("git", "show", f"{commit}:VERSION").decode().strip() == version,
        "Commit version differs",
    )
    jobs = github_pages(
        f"actions/runs/{run_id}/attempts/{run['run_attempt']}/jobs?per_page=100", "jobs"
    )
    require(
        jobs
        and all(job["conclusion"] == "success" for job in jobs)
        and any(job["name"] == "build" for job in jobs)
        and any(job["name"].startswith("python (") for job in jobs)
        and any(job["name"].startswith("javascript (") for job in jobs),
        "Every build and installation job must pass",
    )
    artifacts = github_pages(
        f"actions/runs/{run_id}/artifacts?per_page=100", "artifacts"
    )
    artifacts = [
        artifact for artifact in artifacts if artifact["name"] == "rime-api-dist"
    ]
    require(
        len(artifacts) == 1 and not artifacts[0]["expired"],
        "A unique unexpired rime-api-dist artifact is required",
    )
    artifact = artifacts[0]
    require(artifact["workflow_run"]["head_sha"] == commit, "Artifact commit differs")
    data = command(
        "gh", "api", f"repos/{REPOSITORY}/actions/artifacts/{artifact['id']}/zip"
    )
    require(
        artifact["digest"] == "sha256:" + hashlib.sha256(data).hexdigest(),
        "GitHub artifact checksum differs",
    )
    with zipfile.ZipFile(io.BytesIO(data)) as archive:
        require(
            len(archive.namelist()) == len(set(archive.namelist())),
            "Duplicate artifact files",
        )
        files = {name: archive.read(name) for name in archive.namelist()}
    schemas = {
        name: command("git", "show", f"{commit}:schema/{name}")
        for name in ("rime/text_to_speech.proto", "text_to_speech.asyncapi.yaml")
    }
    validate_archives(files, version, schemas)
    directory.mkdir(parents=True, exist_ok=False)
    for name, contents in files.items():
        (directory / name).write_bytes(contents)
    record = {
        "version": version,
        "commit": commit,
        "run_id": run_id,
        "artifact_id": artifact["id"],
        "artifact_digest": artifact["digest"],
        "sha256": {
            name: hashlib.sha256(contents).hexdigest()
            for name, contents in files.items()
        },
    }
    (directory / "RELEASE.json").write_text(json.dumps(record, indent=2) + "\n")
    print(f"Verified {version} from {commit}, test run {run_id}")


def fetch(url):
    with urlopen(url, timeout=30) as response:
        return response.read()


def registry_metadata(registry, version):
    url = (
        f"https://registry.npmjs.org/@rimelabs%2fapi/{version}"
        if registry == "npm"
        else f"https://pypi.org/pypi/rime-api/{version}/json"
    )
    try:
        return json.loads(fetch(url))
    except HTTPError as error:
        if error.code == 404:
            return None
        raise


def compare_registry(registry, metadata, files, version):
    """Return matching published files and their URLs; reject version conflicts."""
    if metadata is None:
        return {}
    names = archive_names(version)[registry]
    if registry == "npm":
        require(
            metadata["name"] == "@rimelabs/api" and metadata["version"] == version,
            "Unexpected npm version",
        )
        integrity = (
            "sha512-"
            + base64.b64encode(hashlib.sha512(files[names[0]]).digest()).decode()
        )
        require(
            metadata["dist"]["integrity"] == integrity,
            "npm already has different bytes for this version",
        )
        return {names[0]: metadata["dist"]["tarball"]}
    require(metadata["info"]["version"] == version, "Unexpected PyPI version")
    published = {}
    for distribution in metadata["urls"]:
        name = distribution["filename"]
        require(
            name in names, "PyPI already has unexpected distributions for this version"
        )
        require(
            distribution["digests"]["sha256"]
            == hashlib.sha256(files[name]).hexdigest(),
            "PyPI already has different bytes for this version",
        )
        published[name] = distribution["url"]
    return published


def check_registry(registry, directory, download):
    record = json.loads((directory / "RELEASE.json").read_text())
    version = record["version"]
    files = {name: (directory / name).read_bytes() for name in record["sha256"]}
    require(
        all(
            hashlib.sha256(data).hexdigest() == record["sha256"][name]
            for name, data in files.items()
        ),
        "Prepared archive checksum differs",
    )
    names = archive_names(version)[registry]
    for attempt in range(12 if download else 1):
        published = compare_registry(
            registry, registry_metadata(registry, version), files, version
        )
        if len(published) == len(names) or not download:
            break
        if attempt < 11:
            time.sleep(10)
    if download:
        require(len(published) == len(names), "Published files are not yet available")
        download.mkdir(parents=True, exist_ok=False)
        for name, url in published.items():
            expected_host = (
                "registry.npmjs.org" if registry == "npm" else "files.pythonhosted.org"
            )
            require(
                urlparse(url).scheme == "https"
                and urlparse(url).hostname == expected_host,
                "Unexpected distribution host",
            )
            data = fetch(url)
            require(
                data == files[name],
                "Downloaded registry archive differs from tested archive",
            )
            (download / name).write_bytes(data)
    print(
        f"{registry}_publish_required={'false' if len(published) == len(names) else 'true'}"
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    preparation = commands.add_parser("prepare")
    preparation.add_argument("--run-id", type=int, required=True)
    preparation.add_argument("--version", required=True)
    preparation.add_argument("--directory", type=Path, required=True)
    registry = commands.add_parser("registry")
    registry.add_argument("--registry", choices=["npm", "pypi", "both"], required=True)
    registry.add_argument("--directory", type=Path, required=True)
    registry.add_argument("--download", type=Path)
    arguments = parser.parse_args()
    if arguments.command == "prepare":
        prepare(arguments.run_id, arguments.version, arguments.directory)
    else:
        require(
            not (arguments.registry == "both" and arguments.download),
            "Download one registry at a time",
        )
        for name in (
            ["npm", "pypi"] if arguments.registry == "both" else [arguments.registry]
        ):
            check_registry(name, arguments.directory, arguments.download)


if __name__ == "__main__":
    main()
