# Contributing to lore

Short, because the rules are few.

## Before you open a pull request

```bash
cli/make
```

It runs `cargo fmt`, `cargo clippy` (any warning fails), `cargo test`,
`cargo build` and `cargo build --release`, and stops at the first failure.
Green means every step passed.

## What a change looks like

- **One logical change per pull request.** A refactor and a behaviour change
  are two.
- **Commit subjects:** `v<x.y>.<N> - <area>: <lowercase imperative>` —
  `v0.1.12 - make: fail on a stale lockfile`. `N` is one more than the last
  commit on `main`; if you cannot know it, write the series and an `X`
  (`v0.1.X - …`) and it is numbered at the merge. The body says why.
- **Tests are named as sentences** that state the rule they defend:
  `a_symlinked_command_is_listed`.
- **Comments say why, not what.**
- **`CHANGELOG.md`** gets a line under `## [Unreleased]`.

## Adding or improving a command's page

The pages lore shows beside the list are Markdown files in
[`content/`](../content/), one per command, built into lore when it is
compiled. [`content/README.md`](../content/README.md) has the format and
what a page must meet before it is sent: links over https only, no log-in
or sign-up asked of anyone, and examples that are safe or say plainly when
they are not. A pull request that adds pages is one change per command, or
one for a small related set.

## Reporting a bug, asking for a change

Open an issue from one of the two templates. A vulnerability is not an
issue: [SECURITY.md](SECURITY.md) says where it goes.

## Conduct

[CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
