# Changelog

Every notable change to this repository, newest first. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Versioning is
[semantic](https://semver.org/spec/v2.0.0.html) in shape, but everything below
1.0.0 is prototype work: nothing here is stable yet.

Every commit subject starts with its series and a running count,
`v<x.y>.<N>` — so `v0.1.7` is the seventh commit of the 0.1 series, not a
release. A release is an entry here.

## [Unreleased]

### Added

- `lore`: a terminal list of every command you can type, for Linux and macOS
  — the executables on `$PATH` and in the directories it often leaves out
  (`/usr/sbin`, Homebrew, MacPorts, Nix, Snap, Flatpak, `~/bin`, language
  tools, `/opt/*/bin`, macOS's `/etc/paths`; symlinked ones too), the
  builtins and keywords of the user's shell (bash, zsh or fish), and the
  user's aliases from the shell's system-wide and own startup files (fish
  functions too), shown with what they run and what they hide — each name
  once, sorted A to Z under section letters (`#` for names that do not start
  with a letter). Missing places are skipped; the lists live in
  `src/compile_config.rs`.
- Without a terminal (a pipe, cron, `ssh` without `-t`) it says so in one
  line and exits with status 1, instead of crashing.
- A search bar on top: `/` opens it, typing filters by name, Esc closes it.
  A single character puts its own section first. The keys of the current
  mode are always shown on the bottom line. ↑/↓
  move; Esc (when not searching) and Ctrl+C quit.
- `cli/make` (fmt, clippy, test, build, release, copy to `app/`) and
  `cli/clean` (`cargo clean`).
- The MIT licence, and the repository's pages: the README, `SECURITY.md`,
  `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, issue and pull-request templates,
  and the workflows, which only pass for now.
