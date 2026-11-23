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

- `lore`: a terminal list of every executable on `$PATH`, sorted A to Z under
  section letters (`#` for names that do not start with a letter). ↑/↓ move,
  Ctrl+C quits. Symlinked commands are included.
- `cli/make` (fmt, clippy, test, build, release, copy to `app/`) and
  `cli/clean` (`cargo clean`).
- The MIT licence, and the repository's pages: the README, `SECURITY.md`,
  `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, issue and pull-request templates,
  and the workflows, which only pass for now.
