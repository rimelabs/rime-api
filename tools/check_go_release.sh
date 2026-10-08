#!/usr/bin/env bash
# Verify the module from the public proxy, with no local module replacements.
set -euo pipefail
version=${1:?Supply the stable API version}
commit=${2:?Supply the tested commit}
[[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]
[[ "$commit" =~ ^[0-9a-f]{40}$ ]]
repository=$(git rev-parse --show-toplevel)
module=github.com/rimelabs/rime-api/go
work=$(mktemp -d)
trap 'chmod -R u+w "$work"; rm -rf "$work"' EXIT
export GOENV=off GOWORK=off GOFLAGS= GOTOOLCHAIN=auto
export GOPROXY=https://proxy.golang.org GOSUMDB=sum.golang.org
export GOPRIVATE= GONOPROXY= GONOSUMDB=
export GOPATH="$work/gopath" GOMODCACHE="$work/modules"
export GIT_TERMINAL_PROMPT=0
cd "$work"
go mod init example.com/rime-api-release-check
for attempt in {1..40}; do
	if go mod download -json "$module@v$version" >module.json; then
		break
	fi
	if [[ "$attempt" == 40 ]]; then
		echo 'Go publication is not available. Retry the original release run.' >&2
		exit 1
	fi
	echo "Waiting for Go publication, attempt $attempt of 40." >&2
	sleep 15
done
jq -e --arg version "v$version" '.Version == $version' module.json >/dev/null
module_directory=$(jq -er '.Dir | select(type == "string" and length > 0)' module.json)
# Origin metadata is optional. Compare the downloaded files with the tested tree.
mkdir "$work/expected"
git -C "$repository" archive "$commit" go | tar -x -C "$work/expected"
diff -r "$work/expected/go" "$module_directory"
go get "$module@v$version"
cat >main.go <<'GO'
package main
import (
  "github.com/rimelabs/rime-api/go"
  "google.golang.org/protobuf/proto"
)
func main() {
  message := &rimeapi.SynthesisRequest{Text: "release check"}
  wire, err := proto.Marshal(message)
  if err != nil { panic(err) }
  decoded := new(rimeapi.SynthesisRequest)
  if err := proto.Unmarshal(wire, decoded); err != nil { panic(err) }
  if !proto.Equal(message, decoded) { panic("round trip differs") }
  _ = rimeapi.NewTextToSpeechClient
  _ = rimeapi.NewSpeechToTextClient
}
GO
go mod tidy
CGO_ENABLED=0 go run .
go list -m "$module"
