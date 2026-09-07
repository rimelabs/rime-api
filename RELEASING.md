# Release packages

Python `rime-api` and npm `@rimelabs/api` share the version in `VERSION`.
Package publication is manual. CI builds and tests archives but does not publish.

## Prepare a release

1. Merge the required schema sync and package changes into `main`.
2. Update `VERSION` in a PR. Use a `major.minor.patch` version.
3. Run the checks and merge the version change:

   ```shell
   bazel test //tests/...
   bash tools/check_compatibility.sh
   bazel run //:format
   uv run --no-project --python 3.13 python tools/check_release.py "v$(cat VERSION)"
   ```

4. Wait for all jobs in `Check packages` to pass on the release commit.
   Download its `rime-api-dist` artifact and extract it into `dist/`.
   Use that exact commit for the following commands.

The build uses the committed schemas. A source repository update does not
change a release until its Copybara PR is merged here. `SOURCE.json` records
the release version and SHA-256 hashes of both definitions. The source
revision is in the schema sync commit's `GitOrigin-RevId` trailer.

## Test local archives

These commands can also check an archive before publication:

```shell
version=$(cat VERSION)
uv run --isolated --no-project --python 3.10 \
  --with "./dist/rime_api-$version-py3-none-any.whl" \
  --with protobuf==6.33.5 \
  python tests/packages_test.py --fixtures tests/fixtures.json
uv run --isolated --no-project --python 3.14 \
  --with "./dist/rime_api-$version.tar.gz" \
  --with protobuf==7.36.1 \
  python tests/packages_test.py --fixtures tests/fixtures.json
bash tests/test_installed_javascript.sh "dist/rime-api-$version.tgz"
```

## Publish

Rime must control the PyPI name `rime-api` and npm scope `@rimelabs` before the
first release. Set `UV_PUBLISH_TOKEN` through your secret manager and use
`npm login --registry=https://registry.npmjs.org` with an account that can
publish `@rimelabs/api`.

Publish the tested archives from the selected CI run:

```shell
version=$(cat VERSION)
uv run --no-project --python 3.13 python tools/check_release.py "v$version"
uv publish --trusted-publishing never \
  "dist/rime_api-$version-py3-none-any.whl" \
  "dist/rime_api-$version.tar.gz"
npm publish "dist/rime-api-$version.tgz" --access public --registry=https://registry.npmjs.org
```

After both registries accept the version, tag the release commit:

```shell
git tag -a "v$version" -m "Rime API $version"
git push origin "v$version"
```

If one registry fails, publish the same tested version to the missing registry.
Do not rebuild different contents under a version that already exists. Restore
both registries before starting the next release.

## Compatibility

CI compares Protobuf changes against the PR base and the latest ancestor
release tag. It applies Buf's `FILE` rules to check the generated API as well
as the wire format. The first release has no release baseline.

Keep field numbers, field names, message names, and enum values compatible.
Keep the Protobuf package `rime` and source path `rime/text_to_speech.proto`.
The distribution names and Python import name do not change the wire protocol.

Review AsyncAPI behavior changes with the engine change. Buf checks Protobuf;
it does not establish compatibility of WebSocket connection behavior.
