#!/usr/bin/env bash
set -euo pipefail

repository=$(cd "$(dirname "$0")/.." && pwd)
cd "$repository"
bazel run @rules_buf_toolchains//:buf -- lint "$repository"

if ! git rev-parse --verify HEAD >/dev/null 2>&1; then
	echo 'Schema lint passed. Make the initial commit to enable Git baseline checks.'
	exit 0
fi

references=()
if [[ -n "${BASE_SHA:-}" ]]; then
	references+=("$BASE_SHA")
fi
while IFS= read -r tag; do
	if [[ "$tag" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]] &&
		[[ $(git rev-list -n 1 "$tag") != $(git rev-parse HEAD) ]]; then
		references+=("$tag")
		break
	fi
done < <(git tag --merged HEAD --list 'v*' --sort=-version:refname)

if [[ ${#references[@]} -eq 0 ]]; then
	echo 'Schema lint passed. There is no previous release or PR base to compare.'
	exit 0
fi

baseline=$(mktemp -d)
trap 'rm -rf "$baseline"' EXIT
mkdir -p "$baseline/schema/rime"
cp buf.yaml "$baseline/buf.yaml"
for reference in "${references[@]}"; do
	git show "$reference:schema/rime/text_to_speech.proto" >"$baseline/schema/rime/text_to_speech.proto"
	bazel run @rules_buf_toolchains//:buf -- breaking "$repository" --against "$baseline"
done
