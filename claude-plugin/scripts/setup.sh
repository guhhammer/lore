#!/usr/bin/env bash
#
# setup.sh — get lore ready for the plugin's skill.
#
#   setup.sh            install lore if it is missing or too old to have
#                       `lore search`, else say where it is
#   setup.sh --update   install the latest release even if lore is there
#   setup.sh --session  the session-start check: the same as no argument,
#                       in one line when lore is ready (its path, for the skill)
#
# It installs for this user only, with no sudo: the release tarball for this
# system from https://github.com/guhhammer/lore/releases, checked against the
# release's SHA256SUMS, the binary into ~/.local/bin and the manual into
# ~/.local/share/man/man1. Everything it says goes to stdout, for the agent
# to read. LORE_REPO (another place with the same release layout, a fork)
# and LORE_TAG (a release, v0.3.0, instead of the latest) change where from.
# A package (.deb, .rpm, AUR, Homebrew) installed by hand works too:
# then this script finds it and changes nothing.

set -euo pipefail

REPO="${LORE_REPO:-https://github.com/guhhammer/lore}"
BIN_DIR="${HOME}/.local/bin"
MAN_DIR="${HOME}/.local/share/man/man1"
mode="${1:-}"

# the lore to use: the first, of the one on PATH and the one this script
# installs, that has `lore search`; else whichever there is. (Another
# program called lore, an AUR pager, can be on PATH too.)
find_lore() {
    local candidates=() c
    c=$(command -v lore 2>/dev/null) && candidates+=("$c")
    [ -x "$BIN_DIR/lore" ] && candidates+=("$BIN_DIR/lore")
    for c in "${candidates[@]}"; do
        if ready "$c"; then
            echo "$c"
            return
        fi
    done
    for c in "${candidates[@]}"; do
        if "$c" --version 2>/dev/null | grep -q "^lore [0-9]"; then
            echo "$c"
            return
        fi
    done
}

# a lore new enough for the skill: it has `lore search`
ready() {
    "$1" --help 2>/dev/null | grep -q "lore search"
}

say_ready() {
    local lore="$1"
    echo "lore is ready: $lore ($("$lore" --version 2>/dev/null)); the lore skill runs it by this path."
    case ":$PATH:" in
        *":$(dirname "$lore"):"*) ;;
        *) echo "Its folder is not on PATH: call it as $lore, or add $(dirname "$lore") to PATH." ;;
    esac
}

target() {
    local os arch
    case "$(uname -s)" in
        Linux) os="unknown-linux-musl" ;;
        Darwin) os="apple-darwin" ;;
        *) return 1 ;;
    esac
    case "$(uname -m)" in
        x86_64 | amd64) arch="x86_64" ;;
        aarch64 | arm64) arch="aarch64" ;;
        *) return 1 ;;
    esac
    echo "$arch-$os"
}

sha256() {
    if command -v sha256sum >/dev/null; then
        sha256sum "$1" | cut -d' ' -f1
    else
        shasum -a 256 "$1" | cut -d' ' -f1
    fi
}

install_latest() {
    local triple tag version name expected actual
    if ! triple=$(target); then
        echo "lore has no release for $(uname -s) $(uname -m); build it from source: $REPO#build-from-source"
        return 1
    fi
    command -v curl >/dev/null || { echo "curl is needed to download lore; install curl, then run /lore:setup."; return 1; }
    # the latest release's tag, from where /releases/latest redirects (no API
    # call, so no rate limit)
    tag="${LORE_TAG:-}"
    if [ -z "$tag" ]; then
        tag=$(curl -fsSIL --max-time 30 -o /dev/null -w '%{url_effective}' "$REPO/releases/latest" 2>/dev/null | sed 's|.*/tag/||') || true
    fi
    case "$tag" in
        v[0-9]*) ;;
        *) echo "Could not find lore's latest release (offline?). Run /lore:setup later."; return 1 ;;
    esac
    version="${tag#v}"
    # already the latest, only without `lore search`: nothing newer to get
    if [ "$mode" != "--update" ] && [ -n "$lore" ] && [ "$("$lore" --version 2>/dev/null)" = "lore $version" ]; then
        echo "lore $version at $lore is the latest release; it has no \`lore search\` yet, so the skill uses lore -p and lore explain."
        return 1
    fi
    name="lore-$tag-$triple"
    dir=$(mktemp -d) # removed on exit, see below
    curl -fsSL --max-time 120 -o "$dir/$name.tar.gz" "$REPO/releases/download/$tag/$name.tar.gz"
    curl -fsSL --max-time 30 -o "$dir/SHA256SUMS" "$REPO/releases/download/$tag/SHA256SUMS"
    expected=$(awk -v f="$name.tar.gz" '$2 == f || $2 == "*" f { print $1 }' "$dir/SHA256SUMS")
    actual=$(sha256 "$dir/$name.tar.gz")
    if [ -z "$expected" ] || [ "$expected" != "$actual" ]; then
        echo "The download of $name.tar.gz does not match the release's SHA256SUMS; nothing was installed."
        return 1
    fi
    tar -xzf "$dir/$name.tar.gz" -C "$dir"
    mkdir -p "$BIN_DIR" "$MAN_DIR"
    install -m 755 "$dir/$name/lore" "$BIN_DIR/lore"
    install -m 644 "$dir/$name/lore.1" "$MAN_DIR/lore.1"
    echo "Installed lore $version into $BIN_DIR (checksum verified; the manual into $MAN_DIR)."
}

dir=""
trap '[ -z "$dir" ] || rm -rf "$dir"' EXIT

lore=$(find_lore)
if [ "$mode" != "--update" ] && [ -n "$lore" ] && ready "$lore"; then
    say_ready "$lore"
    exit 0
fi
if [ -n "$lore" ] && [ "$mode" != "--update" ]; then
    echo "lore at $lore has no \`lore search\` (too old for this plugin); looking for a newer release."
fi
if install_latest; then
    lore="$BIN_DIR/lore" # the one just installed, whatever PATH finds first
    if [ -n "$lore" ] && ready "$lore"; then
        say_ready "$lore"
    else
        echo "The latest lore release does not have \`lore search\` yet: the skill uses lore -p and lore explain until it does."
    fi
else
    # the session goes on either way; the skill says what is missing
    exit 0
fi
