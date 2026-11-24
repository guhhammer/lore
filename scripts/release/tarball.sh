#!/usr/bin/env bash
#
# tarball.sh TARGET — pack the release binary built for TARGET, its man page,
# the license and the README into dist/lore-<version>-<TARGET>.tar.gz.
# Run from the repository root after `cargo build --release --target TARGET`.

set -euo pipefail

target=${1:?usage: tarball.sh TARGET}
version=$(sed -n 's/^version = "\(.*\)"$/\1/p' Cargo.toml | head -n 1)
name="lore-v$version-$target"

mkdir -p dist "dist/$name"
cp "target/$target/release/lore" man/lore.1 LICENSE "dist/$name/"
cp .github/README.md "dist/$name/README.md"
tar -czf "dist/$name.tar.gz" -C dist "$name"
rm -r "dist/$name"
echo "dist/$name.tar.gz"
