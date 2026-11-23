# lore

A terminal list of every command on your PATH, grouped A to Z.

`lore` reads every directory on your `$PATH`, collects the executables in
them, and shows them in one scrollable list, sorted alphabetically under the
section letters `A` to `Z` (names that start with a digit or a symbol go under
`#`). It is for anyone who wants to see what commands their machine has.

It is one Rust binary built on [ratatui](https://ratatui.rs): `src/main.rs`
scans `$PATH` once at start, builds the rows, and draws them.

```text
lore/
├── src/main.rs          the whole program: scan $PATH, group, draw, read keys
├── cli/                 make (fmt, clippy, test, build, release → app/) and clean
├── app/                 where cli/make copies the release binary
└── .github/             this page, the policies, the templates, the workflows (paused)
```

## Quick start

```bash
cargo run --release
```

A bordered list titled `commands (<count>)` fills the terminal. ↑ and ↓ move
through the commands (the letter headers are skipped); Ctrl+C quits.

## Building

```bash
cli/make     # cargo fmt, clippy, test, build, build --release, then copies app/lore
cli/clean    # cargo clean
```

Needs Rust stable, edition 2024 (rustc 1.85 or newer).

## Where to read next

| | |
| --- | --- |
| [CHANGELOG.md](../CHANGELOG.md) | every release |
| [SECURITY.md](SECURITY.md) | reporting a vulnerability |
| [CONTRIBUTING.md](CONTRIBUTING.md) | before a pull request |

## Licence

MIT. See [LICENSE](../LICENSE).
