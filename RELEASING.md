# Release packages

Python `rime-api` and npm `@rimelabs/api` share the version in `VERSION`.
`Check packages` builds and tests the archives. `Release packages` publishes
those same archives through GitHub Actions Trusted Publishing.

## Set up registry access once

Create the GitHub environments `npm-release` and `pypi-release`. For each,
set deployment branches to selected branches and add only `main`.
The release workflow also requires `main`.

For npm, the package must exist before you can add a trusted publisher.
The initial `@rimelabs/api@0.0.1` upload is complete. In the package settings,
add a GitHub Actions trusted publisher with these values:

| Setting | Value |
| --- | --- |
| Organization or user | `rimelabs` |
| Repository | `rime-api` |
| Workflow filename | `release.yaml` |
| Environment | `npm-release` |
| Permission | Publish |

With npm 11.15 or later, an account with package write access and 2FA can
also configure this from the terminal:

```shell
npm trust github @rimelabs/api --file release.yaml \
  --repository rimelabs/rime-api --environment npm-release --allow-publish \
  --registry=https://registry.npmjs.org
```

For PyPI, sign in and open <https://pypi.org/manage/account/publishing/>.
Add a pending GitHub publisher for the new project `rime-api`, with owner
`rimelabs`, repository `rime-api`, workflow `release.yaml`, and environment
`pypi-release`. If the project already exists, add the publisher in its
Publishing settings instead. A pending publisher does not reserve the name;
the first successful upload creates the project.

No npm or PyPI token is required in GitHub secrets. The workflow receives
short-lived credentials through OIDC. npm publication uses Node.js 24 on a
GitHub-hosted runner. npm provenance is enabled only when the source
repository is public.

See the [npm trusted publisher guide](https://docs.npmjs.com/trusted-publishers/)
and [PyPI pending publisher guide](https://docs.pypi.org/trusted-publishers/creating-a-project-through-oidc/).

## Prepare a release

1. Merge the required schema sync and package changes into `main`.
2. Update `VERSION` in a PR. Use a `major.minor.patch` version.
3. Run `bazel run //:format`, `bazel test //tests/...`, and
   `bash tools/check_compatibility.sh`. Merge after all PR checks pass.
4. Wait for every job in `Check packages` to pass on the commit in `main`.
   Copy the run ID from its URL: `/actions/runs/<run_id>`.

The build uses the committed schemas. A source repository update does not
change a release until its Copybara PR is merged here. `SOURCE.json` records
the version and SHA-256 hashes of both definitions. The source revision is
in the schema sync commit's `GitOrigin-RevId` trailer.

## Publish from GitHub

1. Open this repository's **Actions** tab and select **Release packages**.
2. Select **Run workflow**. Leave the branch as `main`.
3. Enter the successful `Check packages` run ID and its version.
4. Select `npm`, `pypi`, or `both`, then start the workflow.

Version `0.0.1` is published to both registries and tagged as `v0.0.1`.
Its archives came from run `34162808003`, which tested commit
`41029e4a2cbdd3f1146d0def0abe1c9bde8bd806`. The release workflow verified
[npm](https://github.com/rimelabs/rime-api/actions/runs/34165338107) and
[PyPI](https://github.com/rimelabs/rime-api/actions/runs/34168990251).
To repeat verification, select that original run, version `0.0.1`, and `both`.

The workflow checks the source repository, branch, workflow, commit,
version, job results, artifact checksum, package metadata, and schemas.
It downloads the selected run's `rime-api-dist` artifact; it does not rebuild.
The selected commit must be an ancestor of the workflow commit on `main`.
Expired artifacts cannot be used.

Before publication, it compares any existing registry files with the tested
archives. Identical files are skipped. Different contents for the same
version stop the release. After publication, the workflow downloads the
registry files, checks their contents, and tests Python imports and message
serialization, plus JavaScript ESM, CommonJS, and TypeScript use.

The run retains the archives and a `RELEASE.json` record in its `release-dist`
artifact for 90 days. The record includes the source commit, test run,
original artifact identity, and archive checksums.

If one upload or installation check fails, correct the cause and start the
workflow again with the **same run ID and version**. Select only the failed
registry if the other has passed. uv skips identical Python files if an
upload stopped between the wheel and source archive. Do not build different
contents under an existing version.

After both registry jobs pass, tag the **selected release commit**:

```shell
version=0.0.1
release_commit=41029e4a2cbdd3f1146d0def0abe1c9bde8bd806
git tag -a "v$version" "$release_commit" -m "Rime API $version"
git push origin "v$version"
```

Replace both values for each later release. Do not tag the workflow commit
unless it is also the selected release commit.

## Manual publication if needed

Download the selected successful run's `rime-api-dist` artifact into `dist/`.
Use a PyPI token from your secret manager through `UV_PUBLISH_TOKEN`, and an
npm account with package write access and 2FA:

```shell
version=0.0.1
uv publish --trusted-publishing never \
  "dist/rime_api-$version-py3-none-any.whl" \
  "dist/rime_api-$version.tar.gz"
npm publish "./dist/rime-api-$version.tgz" --access public \
  --registry=https://registry.npmjs.org \
  --@rimelabs:registry=https://registry.npmjs.org
```

The scope-specific npm option overrides the GitHub Packages setting used for
the internal UI package for this command. Test installation from the registries
before you create the tag.

## Compatibility

CI compares Protobuf changes against the PR base and the latest ancestor
release tag. It applies Buf's `FILE` rules to check the generated API as well
as the wire format. The first release has no release baseline.

Keep field numbers, field names, message names, and enum values compatible.
Keep the Protobuf package `rime` and source path `rime/text_to_speech.proto`.
The distribution names and Python import name do not change the wire protocol.

Review AsyncAPI behavior changes with the engine change. Buf checks Protobuf;
it does not establish compatibility of WebSocket connection behavior.
