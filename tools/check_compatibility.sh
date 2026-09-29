#!/usr/bin/env bash
set -euo pipefail

repository=$(cd "$(dirname "$0")/.." && pwd)
cd "$repository"
bazel build //schema:workspace @rules_buf_toolchains//:buf
workspace=$(bazel cquery --output=files //schema:workspace)
buf=$(bazel cquery --output=files @rules_buf_toolchains//:buf)
execution_root=$(bazel info execution_root)
output_base=$(bazel info output_base)
report=${COMPATIBILITY_REPORT:-$(mktemp)}
bazel run //tools:check_compatibility -- \
	--buf "$output_base/$buf" \
	--workspace "$execution_root/$workspace" \
	--repository "$repository" \
	--base-revision "${BASE_SHA:-}" \
	--report "$report"
if [[ -n "${GITHUB_STEP_SUMMARY:-}" ]]; then
	cat "$report" >>"$GITHUB_STEP_SUMMARY"
fi
