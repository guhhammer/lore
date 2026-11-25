#!/usr/bin/env bash
#
# crate.sh [ARGS] — publish lore-tui to crates.io: pack content/ into
# content.zst (crates.io takes at most 30,000 files; content/ has about
# 41,000), then `cargo publish ARGS`. build.rs unpacks content.zst when there
# is no content/, as in the published crate. Run from the repository root:
#
#   scripts/release/crate.sh --dry-run   build and check it as crates.io would
#   scripts/release/crate.sh             publish
#
# Needs the zstd program. content.zst is gitignored: made again each time.

set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/../.."

# publish only what is committed: cargo would count content.zst, made below,
# as a change, so the check is made here and cargo is told to allow it
if [ -n "$(git status --porcelain)" ]; then
  echo "crate.sh: commit or stash your changes first:" >&2
  git status --short >&2
  exit 1
fi

# one record a page: 0x1E, its path under content/, a newline, the page
{
  find content -type f -name '*.md' -print0 | sort -z |
    while IFS= read -r -d '' page; do
      printf '\036%s\n' "${page#content/}"
      cat "$page"
    done
} | zstd -19 -q -f -o content.zst
echo "content.zst: $(find content -type f -name '*.md' | wc -l) pages, $(du -h content.zst | cut -f1)"

cargo publish --allow-dirty "$@"
