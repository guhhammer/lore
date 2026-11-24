#!/usr/bin/env bash
#
# notes.sh TAG — the release notes for TAG (v0.1.3): its CHANGELOG.md section
# (or the Unreleased one, when it has none), then how to install each file.

set -euo pipefail

tag=${1:?usage: notes.sh TAG}
version=${tag#v}

# the lines between a "## [...]" heading and the next one
section() {
  awk -v head="## [$1]" '
    index($0, head) == 1 { on = 1; next }
    /^## \[/ && on       { exit }
    on
  ' CHANGELOG.md
}

# the version's own section; when the changes are still under
# "## [Unreleased]" (a tag pushed without renaming it), those, so that a
# release is just a tag
notes=$(section "$version")
if [[ -z ${notes//[[:space:]]/} ]]; then
  notes=$(section Unreleased)
fi
printf '%s\n' "$notes"

cat <<NOTES

## Install

| System | File | How |
| --- | --- | --- |
| Debian, Ubuntu, Mint, Pop!_OS | \`lore_${version}-1_amd64.deb\` (ARM: \`_arm64\`) | \`sudo apt install ./lore_${version}-1_amd64.deb\` |
| Fedora, RHEL, Rocky, Alma, openSUSE | \`lore-${version}-1.x86_64.rpm\` (ARM: \`.aarch64\`) | \`sudo dnf install ./lore-${version}-1.x86_64.rpm\` |
| Any Linux | \`lore-${tag}-x86_64-unknown-linux-musl.tar.gz\` (ARM: \`aarch64-…\`) | unpack, copy \`lore\` into \`~/.local/bin\` |
| macOS, Apple Silicon | \`lore-${tag}-aarch64-apple-darwin.tar.gz\` | unpack, copy \`lore\` into your PATH |
| macOS, Intel | \`lore-${tag}-x86_64-apple-darwin.tar.gz\` | the same |

The macOS binaries are not signed: the first time, run
\`xattr -d com.apple.quarantine lore\`, or macOS refuses to open it.
\`SHA256SUMS\` has every file's checksum (\`sha256sum -c SHA256SUMS\`).
NOTES
