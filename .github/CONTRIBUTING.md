# Contributing to lore

Thanks for helping. The rules are few, and most contributions are a single
file.

## The easiest ways to help

| You want to… | Do this | Guide |
| --- | --- | --- |
| Fix or improve a command's page | Edit `content/NAME.md` | [content/README.md](../content/README.md) |
| Add a task page ("free disk space") | Add a file to `recipes/` | [recipes/README.md](../recipes/README.md) |
| Add a colour palette | Add a file to `palettes/` | [palettes/README.md](../palettes/README.md) |
| Report a bug or ask for something | Open an issue from a template | — |
| Report a security problem | **Not** an issue: report it privately | [SECURITY.md](SECURITY.md) |

## Before you open a pull request

Run the checks:

```bash
cli/make
```

It runs `cargo fmt`, `cargo clippy` (any warning fails), the tests, and the
debug and release builds, and stops at the first failure. Green means it is
ready. The same checks run on GitHub for every pull request.

## What a good change looks like

- **One change per pull request.** A refactor and a behaviour change are
  two. Pages: one per command, or a small related set.
- **Commit subjects** follow `v<x.y>.<N> - <area>: <lowercase imperative>`,
  for example `v0.1.12 - make: fail on a stale lockfile`. `N` is one more
  than the last commit on `main`; if you can't know it, write `X`
  (`v0.1.X - …`) and it is numbered when merged. The body says *why*.
- **Every new feature comes with tests and docs**, and a line in
  `CHANGELOG.md` under `## [Unreleased]`.
- **Tests live in `tests/`** and are named as sentences that state the rule
  they defend: `a_symlinked_command_is_listed`.
- **Every constant and fixed text lives in `src/compile_config.rs`**, not
  scattered through the code.
- **Comments say why, not what.**

## Rules lore keeps

These protect the people who use lore; a change that breaks one will not be
merged.

- lore **never runs the commands it lists**. It only runs `man`, `whatis`
  and package-manager queries, with plain arguments, never through a shell.
- **A missing file or folder is skipped**, never an error.
- **Links are `https://` only**, and lore never opens them itself.
- **No page asks anyone to log in** or sign up anywhere.

## Conduct

Be kind: [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
