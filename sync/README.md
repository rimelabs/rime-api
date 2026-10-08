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
`main`. The commit uses a fixed public message and author. It does not copy
private commit descriptions or author addresses.

## Source revision and merge checks

`sync/SOURCE_REVISION` records the exact `rime` commit represented by all exported
schemas. The sync job passes this revision to Copybara, keeps its last-revision
consistency check enabled, and updates the file from the exported commit's
`GitOrigin-RevId` trailer. The revision file survives squash merges.
`sync/INITIAL_REVISION` is the historical bootstrap record; it no longer selects
the current baseline.

The baseline is `a7f0a964c107164a12e6f0647944a5d63345e09a`. All four schemas were
verified byte for byte against that source revision. The earlier TTS-only revision
could not describe the STT files added in PR #12.

The **Verify schema source** workflow checks each proposed schema against its
recorded source revision. It also checks that the revision belongs to `rime/main`.
It posts the **Schema source** commit status. Require this status before merging
into `main`.

This workflow runs trusted code from `main`. It reads candidate files as data
through the GitHub API. It never checks out or executes candidate code with the
private-source credential, and it does not upload private source files or print
schema differences. Normal package checks do not receive that credential. Fork PRs require a
maintainer to dispatch this source check before it accesses the private source.

Schema edits belong in `rime`; export them through Copybara. If an existing export
must be recovered, first compare every exported file with the proposed source
revision. Change `sync/SOURCE_REVISION` only when every file matches. Do not disable
Copybara's consistency check or use `--force` to hide a mismatch.

To add or remove exported files, open a reviewed PR that updates both literal
file lists in `copy.bara.sky`, the selected source revision, and the schema files.
The normal source check rejects an export-list change. After reviewing the new
list for public disclosure, a maintainer can run **Verify schema source** from
`main`, supply the PR number, and select **I reviewed the changed list of public
schema exports**. This verifies all selected files without executing the proposed
Copybara configuration. The approval applies only to that checked commit; a new
push runs the normal check again. Other Copybara behavior changes need a separate
trusted policy review.

Release Please starts the source check explicitly for its own PRs. To recheck
another PR, dispatch **Verify schema source** from `main` with its PR number.
Publication remains separate from schema sync and validation.

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

Pass `--last-rev="$(cat sync/SOURCE_REVISION)"` to use the recorded baseline.
Exit code 4 means no selected files changed.

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
