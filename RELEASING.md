# Release packages

Python `rime-api` and npm `@rimelabs/api` share the version in `VERSION`.
Release Please proposes version and changelog updates in a release PR.
After that PR is merged, `Check packages` builds and tests its merged commit.
`Release packages` publishes those same archives through GitHub Actions
Trusted Publishing, verifies both registries, then creates the tag and
GitHub release.

## Set up registry access once

In GitHub **Settings > Actions > General > Workflow permissions**, enable
**Allow GitHub Actions to create and approve pull requests**. Release Please
uses `GITHUB_TOKEN` to create PRs and explicitly dispatch their checks. It
does not approve or merge them. No additional release token is required.

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

## Routine release

1. Merge schema syncs and package changes into `main`. Use Conventional
   Commit messages: `fix:` for corrections, `feat:` for features, and
   `feat!:` or a `BREAKING CHANGE:` footer for breaking changes. Keep the
   PR title when using squash merge. Maintenance commits alone do not
   normally start a release.
2. `Prepare release PR` opens or updates one release PR with `VERSION`,
   `.release-please-manifest.json`, and `CHANGELOG.md`. It explicitly starts
   `Check packages` on that branch because PRs made with `GITHUB_TOKEN` do
   not start PR workflows automatically.
3. Review the proposed version and public release notes. Both packages use
   the same stable `major.minor.patch` version. The default sync message is
   `feat(api): sync public API definitions`; inspect the actual schema diff
   before accepting the version increase. While below 1.0, breaking changes
   increase the minor version. Compatibility checks still apply.
4. Wait for checks on the latest release PR commit, then merge the PR.
   Merging authorizes publication to both registries. Do not enable automatic
   merging on release PRs. You can leave the PR open to collect more changes.
5. Wait for `Check packages`, `Select checked release`, and `Release packages`
   to pass. Selection accepts automatic publication only for the exact merged commit
   of the bot's release PR. Ordinary commits, schema sync PRs, failed checks,
   fork runs, and checks on the unmerged release branch do not publish.
6. Confirm that the GitHub release contains the package archives,
   `SOURCE.json`, and `RELEASE.json`. The workflow creates `vX.Y.Z` at the
   tested commit only after both registries pass verification.

`Select checked release` dispatches `release.yaml` with the checked run ID and
version. The publishing workflow holds a concurrency lock for that version
through preparation, both registry jobs, and finalization. Manual recovery
uses the same lock. Separate check runs cannot publish that version concurrently.

Keep `VERSION` as the package version source. The manifest records Release
Please's version and must agree with `VERSION`; package templates continue
to use `@VERSION@`. For a deliberate version override, use Release Please's
`Release-As: X.Y.Z` commit footer and review the resulting release PR.

Release Please only prepares PRs. Its action has `skip-github-release: true`.
The publishing workflow creates the final release and changes the merged
PR label from `autorelease: pending` to `autorelease: tagged`. It then runs
release preparation again to collect changes merged during publication.
Finish or recover a pending release before preparing the next one.

For local changes, run `bazel run //:format`, `bazel test //tests/...`, and
`bash tools/check_compatibility.sh` before requesting review.

The build uses the committed schemas. A source repository update does not
change a release until its Copybara PR is merged here. `SOURCE.json` records
the version and a `schemas` map of paths to SHA-256 hashes for all four public
definitions. Release validation also accepts the older TTS-only record format
when recovering a release from a commit with only TTS definitions. The source revision is
in the schema sync commit's `GitOrigin-RevId` trailer.

## Archive checks

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

The run retains the archives and a `RELEASE.json` record in its `release-dist-X.Y.Z`
artifact for 90 days. The record includes the source commit, test run,
original artifact identity, and archive checksums.

Before publication starts, this artifact reserves the version for that check
run and its original archive. Later attempts compare their record with the
existing GitHub release or prior publishing artifacts. A different check run
or rebuilt artifact stops before registry uploads, even if the package bytes
match. Use the original run ID shown in the error. The final GitHub release
retains the record after CI artifacts expire.

## Failed runs and manual recovery

If release PR preparation fails, fix the cause and run **Prepare release PR**
manually on `main`. It also starts checks on an unchanged open release PR.
If the merged commit's checks fail, fix the cause before retrying those checks.
If source changes are needed before any package was published, merge a reviewed
correction, then manually publish its successful check run. After finalization,
replace `autorelease: pending` with `autorelease: tagged` on the original release
PR and run **Prepare release PR**. Automatic publication requires the original
release PR's exact merged commit, so a correction commit uses manual dispatch.
If a package was already published, keep its contents and recover that release
first. Publish source corrections under a new version.

Registry verification waits up to ten minutes for complete version metadata
and downloadable archives. Progress appears in the job log. A successful upload
can remain unavailable while the registry processes it. A timeout does not mean
the upload failed. Retry with the original check run ID and version after the
files become available. Matching published files are verified and skipped;
checksum conflicts and access errors still stop the release immediately.

If publication or finalization fails:

1. Fix the cause, such as a trusted publisher setting.
2. Open **Actions > Release packages > Run workflow** on `main`.
3. Enter the original successful **Check packages** run ID and its version.
   Use the **same run ID and version**, even if `main` has advanced.
4. Select `npm`, `pypi`, or `both`. Select only the failed registry if the
   other upload passed. Use `both` to retry finalization alone; identical
   registry files are skipped.
5. Check that verification and finalization pass.

Manual dispatch also supports a reviewed main commit outside the automatic
release PR path. It retains the same source, job, archive, and registry checks.

uv skips identical Python files if an upload stopped between the wheel and
source archive. A single-registry recovery verifies the other registry too
before finalization. If that registry has not been published, the workflow
reports the incomplete release and leaves the tag and GitHub release absent.
Run recovery for the remaining registry with the same run ID and version.

Finalization can resume after tag creation or a partial GitHub asset upload.
It refuses a tag at a different commit or an existing asset with different
bytes. It uploads assets to a draft, then publishes the GitHub release. Do not
move a tag or rebuild different contents under an existing package version.
Expired CI artifacts cannot be used for recovery.

The original `0.0.1` archives came from run `34162808003`, which tested commit
`41029e4a2cbdd3f1146d0def0abe1c9bde8bd806`. Both registries and tag `v0.0.1`
already exist. The automation starts from that release baseline.

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
the internal UI package for this command. Run GitHub recovery with the original
run ID, version, and `both` to verify the uploads and create the final release.

## Compatibility

CI compares Protobuf changes against the PR base and the latest ancestor
release tag. It applies Buf's `FILE` rules to check the generated API as well
as the wire format. The first release has no release baseline.

Keep field numbers, field names, message names, and enum values compatible.
Keep the Protobuf package `rime` and source paths `rime/text_to_speech.proto`
and `rime/speech_to_text.proto`.
The distribution names and Python import name do not change the wire protocol.

Review AsyncAPI behavior changes with the engine change. Buf checks Protobuf;
it does not establish compatibility of WebSocket connection behavior.
