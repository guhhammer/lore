#!/usr/bin/env bash
#
# update.sh VERSION — after the release of VERSION (0.1.6) is out, puts its
# version and checksums into packaging/aur/PKGBUILD and
# packaging/homebrew/lore.rb: the source archive's for the AUR, the
# binaries' (from the release's SHA256SUMS) for Homebrew.
# Run from the repository root.

set -euo pipefail

version=${1:?usage: update.sh VERSION}
version=${version#v}
repo=${LORE_REPO_URL:-https://github.com/guhhammer/lore}
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

curl -fsSL -o "$work/SHA256SUMS" "$repo/releases/download/v$version/SHA256SUMS"
curl -fsSL -o "$work/source.tar.gz" "$repo/archive/refs/tags/v$version.tar.gz"
source_sum=$(sha256sum "$work/source.tar.gz" | cut -d' ' -f1)

# the AUR: the version, a new release starts at pkgrel 1, the source's checksum
pkgbuild=packaging/aur/PKGBUILD
sed -i -e "s/^pkgver=.*/pkgver=$version/" -e "s/^pkgrel=.*/pkgrel=1/" \
  -e "s/^sha256sums=.*/sha256sums=('$source_sum')/" "$pkgbuild"

# Homebrew: every version in the file, then each url's checksum on the line
# after it, from SHA256SUMS
formula=packaging/homebrew/lore.rb
sed -i -E "s/v[0-9]+\.[0-9]+\.[0-9]+/v$version/g; s/^  version \".*\"/  version \"$version\"/" "$formula"
while read -r sum file; do
  file=${file#\*}
  [[ $file == *.tar.gz ]] || continue
  # the sha256 line just after the url that ends in this file
  sed -i -E "\\|/$file\"\$|{n;s/sha256 \"[0-9a-f]{64}\"/sha256 \"$sum\"/}" "$formula"
done < "$work/SHA256SUMS"

if grep -q 'sha256 "0\{64\}"' "$formula"; then
  echo "update.sh: a binary of v$version is missing from SHA256SUMS" >&2
  exit 1
fi
echo "PKGBUILD and lore.rb are at $version."
echo "AUR: run makepkg --printsrcinfo > .SRCINFO on Arch, then push both to the lore-tui AUR repository."
echo "Homebrew: copy lore.rb into the tap's Formula/ and push."
