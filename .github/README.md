# lore

A terminal list of every command you can type, grouped A to Z. For Linux
and macOS, with bash, zsh or fish.

`lore` lists every command you can type, each name once, in one scrollable
list sorted alphabetically under the section letters `A` to `Z` (names that
start with a digit or a symbol go under `#`). It is for anyone who wants to
see what commands their machine has, with no internet needed:

- **Programs:** the executables on your `$PATH`, plus the places a `$PATH`
  often leaves out — `/usr/sbin` (where `reboot`, `fdisk` and `useradd` live),
  Homebrew, MacPorts, Nix, Snap, Flatpak, `~/bin`, `~/.cargo/bin`, `~/go/bin`
  and `/opt/*/bin` — and, on macOS, the directories in `/etc/paths`.
- **Your shell's own commands:** the builtins and keywords of bash, zsh or
  fish (`cd`, `export`, `if`…), for the shell in your `$SHELL`.
- **Your aliases,** from your shell's startup files, system-wide and your
  own, each shown with what it runs and, when it shares a name with a
  command, the fact that it hides it. Fish users' functions are listed too.

None of these places has to exist: a missing one is skipped. Every path and
list lives in `src/compile_config.rs`.

It is one Rust binary built on [ratatui](https://ratatui.rs). Aliases are read
from the startup files as text, not by running the shell, so one defined in
a file they load from elsewhere (an oh-my-zsh plugin, say) is not seen; bash
and zsh functions are not listed.

```text
lore/
├── src/                 the program: lib.rs runs it, commands/ finds the commands
├── tests/               the tests, which reach the program through src/lib.rs
├── cli/                 make (fmt, clippy, test, build, release → app/) and clean
├── app/                 where cli/make copies the release binary
└── .github/             this page, the policies, the templates, the workflows (paused)
```

## Quick start

```bash
cargo run --release
```

A search bar sits on top, the list below it, and the keys you can press are
always shown on the bottom line:

- ↑ and ↓ move through the commands (the letter headers are skipped).
- → or Enter opens the selected command's manual page (`man`) beside the
  list, laid out to fit the space left; the list narrows to a tenth of the
  width, and names too long for it end in `...`. ↑ and ↓ scroll the page a
  line, PgUp and PgDn a screen; ← or Esc close it, and the arrows move
  through the commands again. A command with no page shows the likely reasons why.
- `/` opens the search bar; typing keeps only the names that contain what
  you typed, ignoring case. A single character puts its own section first
  (`h` shows the commands starting with `h` on top). Esc leaves the search
  bar but keeps its text and its filter; `/` goes back to it, and Backspace
  empties it.
- Esc, when not searching or reading, quits. Ctrl+C quits at any time.

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
