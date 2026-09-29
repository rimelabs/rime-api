# Schema sync

The authoritative definitions remain in `rimelabs/rime` on `main`.
Change schemas there, then use Copybara to export them here.

| Source in `rime` | Destination in `rime-api` |
| --- | --- |
| `interfaces/rime/text_to_speech.proto` | `schema/rime/text_to_speech.proto` |
| `interfaces/text_to_speech.asyncapi.yaml` | `schema/text_to_speech.asyncapi.yaml` |
| `interfaces/rime/speech_to_text.proto` | `schema/rime/speech_to_text.proto` |
| `interfaces/speech_to_text.asyncapi.yaml` | `schema/speech_to_text.asyncapi.yaml` |

[copy.bara.sky](../copy.bara.sky) lists those exact files. It does not export
the internal model protocol or other files added under `interfaces`.
Build rules, package metadata, tests, and release tooling belong to this
repository and are outside Copybara's destination file list.

Copybara creates a commit on `sync/public-api`. The workflow opens a PR against
`main`. The commit records `GitOrigin-RevId` and uses a fixed public message
and author. It does not copy private commit descriptions or author addresses.
Merge the sync PR with its revision trailer intact. A merge commit or a
fast-forward preserves it. If you squash, retain the trailer in the result.
The fixed public commit message and PR title are
`feat(api): sync public API definitions`. This selects a feature release in
Release Please without copying private source commit descriptions. Review
the proposed version and public changelog in the separate release PR. Use
`fix:` for corrections and `feat!:` for breaking changes when preparing a
squash commit, and retain `GitOrigin-RevId`. Schema validation and package tests must pass. Compatibility changes are
reported for release review and do not stop sync.

## Initial source revision

[INITIAL_REVISION](INITIAL_REVISION) identifies the `rime/main` commit used
for the initial schema files. They are exact copies of that commit. This
baseline lets the first sync proceed before there is a Copybara commit here.
Keep the file as the bootstrap record; later syncs use commit trailers.

The initial snapshot does not depend on the `nastassy/rime-protos` branch or
its PR. It does not include that PR's Arcana deprecation annotations.

The STT files were added as exact copies from source revision
`152718dadfa8faab372ebe4c3e974293330990c4`. The existing TTS files and the
bootstrap revision were retained. Later Copybara exports include all four files.

## Configure the workflow

After the initial repository commit is pushed to `main`, set the repository
secret `RIME_API_SYNC_TOKEN` to a dedicated GitHub token that can read
`rimelabs/rime` and push branches and open PRs in `rimelabs/rime-api`.
Use a dedicated credential, since the normal workflow token cannot read
the private source repository. A token distinct from `GITHUB_TOKEN` also
allows the resulting PR to trigger package CI.

Enable Actions and run `Sync public API` once. It also runs each day. Only
the workflow on `main` uses the sync credential. Normal PR checks need no
access to the private repository.

The workflow updates only `sync/public-api`, opens a PR if needed, and leaves
publication to the release process. Merging the sync PR lets Release Please
open or update the release PR. Merging the release PR starts publication after
checks pass on its merged commit. Do not make manual edits on the sync branch.
Copybara can replace it when a new source change arrives before PR merge.
If Copybara returns exit code 4, the workflow closes the open sync PR and
deletes the sync branch. This removes pending changes that the source has
reverted. Repeated runs succeed when the PR and branch are already absent.
Other Copybara errors stop the workflow before PR or branch cleanup.

## Local use

Validate the configuration and test export behavior without source access:

```shell
bazel run //tools:copybara -- validate "$PWD/copy.bara.sky"
bazel test //tests:copybara_test
```

To preview an export with credentials that can read the source repository:

```shell
bazel run //tools:copybara -- migrate "$PWD/copy.bara.sky" public_api --dry-run
```

Before the first sync commit exists on `main`, add
`--last-rev="$(cat sync/INITIAL_REVISION)"`. Later runs find the source revision
from the last merged Copybara commit. Exit code 4 means no selected files changed.

The integration test uses temporary local Git repositories. It checks first
export, repeated export before and after merge, source reverts, changes,
deletion, metadata, exclusion of private
files, and preservation of destination build files.

## Changes in the monorepo

The package build and sync work without changes to `rime`. The old package PR
can close once this replacement is reviewed. Keep any desired Arcana
deprecation or license-header changes in a separate schema PR.

## Validation and compatibility reports

`rime` is authoritative, including intentional breaking changes. Copybara exports
only the four files listed above. Unrelated source changes produce no sync PR.
Do not change a copied schema here to preserve an older client API.

`tools/check_compatibility.sh` validates the schema with dependencies from the
Bazel build graph. It compares the public Protobuf API with the PR base and the
latest reachable release. Breaking changes produce a warning, a job summary,
and an `api-compatibility` artifact. Invalid schemas, missing imports, tool
errors, and failed package tests still fail CI. Review the AsyncAPI diff too;
Buf checks only Protobuf compatibility.

New source imports may require dependency and package generation changes here.
The build pins Google API definitions and includes the transitive schema files.
Python uses `googleapis-common-protos`; JavaScript includes generated Google RPC
types and uses the well-known types from `@bufbuild/protobuf`.

Before merging a sync PR, review its compatibility report. Use an appropriate
release version and describe required client changes in the release notes.
A compatibility report does not select or publish a release automatically.
