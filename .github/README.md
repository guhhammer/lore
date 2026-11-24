# lore

A terminal list of every command you can type, grouped A to Z. For Linux
and macOS, with bash, zsh or fish.

The app and its command are **lore**. On crates.io its Rust crate is named
**`lore-tui`** (`lore` was taken there); it still installs the `lore`
command.

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
- **Categories** after each name (`[archives files]`), with `danger` in red
  for commands that can destroy data; on those pages, the risky examples
  are marked.
- **Pages in your language** when tldr-pages has a translation (41
  languages, picked from `$LANG`).
- **Subcommands** with a page, such as `git commit` or `docker run`, under
  their program when it is installed.
- **Your own use:** your bookmarks (`b`), the pages you opened last and the
  commands your shell history runs most head the list; in a search, the
  ones you run most come first, and a dim dot marks those you never ran.
  The history is only read, on your machine (`history = off` in
  `~/.config/lore/config` turns it off); the lists are kept in
  `~/.local/share/lore/`.
- **FILES** on each page: the configuration files, logs and data its manual
  names.
- **Colour palettes:** press `p` to pick one; the arrows try it on the whole
  screen, Enter keeps it, and it is saved in `~/.config/lore/config` for the
  next time (`LORE_PALETTE=nord lore` uses another for one run). Eight
  themes, most with a dark and a light variant: `catppuccin-mocha`, `catppuccin-latte`,
  `dracula`, `nord`, `gruvbox-dark`, `gruvbox-light`, `solarized-dark`,
  `solarized-light`, `tokyo-night`, `tokyo-night-day`, `one-dark`,
  `one-light`, `rose-pine`, `rose-pine-dawn`. The default, `terminal`, uses
  your terminal's own colours. Each is a small file in
  [`palettes/`](../palettes/): a new one is a pull request with one file,
  or drop it in `~/.config/lore/palettes/` for yourself.
- **Your aliases,** from your shell's startup files, system-wide and your
  own, each shown with what it runs and, when it shares a name with a
  command, the fact that it hides it. Fish users' functions are listed too.

The pages are Markdown files in [`content/`](../content/), about 8,800 of
them, built into the binary, so `lore` needs no folder of its own to show them. Anyone can add
one: [`content/README.md`](../content/README.md) has the format. Every page
ends its links with the same note: check a link before you open it, and
`lore` will never ask you to log in anywhere or for any account.

None of these places has to exist: a missing one is skipped. Every path and
list lives in `src/compile_config.rs`.

It is one Rust binary built on [ratatui](https://ratatui.rs). Aliases are read
from the startup files as text, not by running the shell, so one defined in
a file they load from elsewhere (an oh-my-zsh plugin, say) is not seen; bash
and zsh functions are not listed.

```text
lore/
├── src/                 the program: lib.rs runs it, commands/ finds the commands
├── content/             the command pages, built into the binary
├── palettes/            the colour palettes, one file each; add yours
├── man/                 lore.1, lore's own manual page
├── tests/               the tests, which reach the program through src/lib.rs
├── cli/                 make (fmt, clippy, test, build, release → app/) and clean
├── app/                 where cli/make copies the release binary
└── .github/             this page, the policies, the templates, the workflows
```

## Install

Each [release](https://github.com/guhhammer/lore/releases) has a `.deb`
(Debian, Ubuntu, Mint, Pop!_OS), an `.rpm` (Fedora, RHEL, Rocky, Alma,
openSUSE) and a static binary that runs on any Linux, for x86_64 and ARM,
and binaries for macOS (Apple Silicon and Intel). The release page says how
to install each one.

## Quick start

```bash
cargo run --release          # the list
cargo run --release -- tar   # tar's page straight away, like `man tar`
```

`lore git commit` opens a subcommand's page the same way, and `lore --print
tar` writes the page out as plain text, for pipes and scripts. `lore NAME` opens NAME's page directly, and closing it quits, as with
`man`. `lore --help` and `lore --version` do what they say. lore's own
manual is `man/lore.1`: `man -l man/lore.1` reads it from the checkout, and
a package installs it so that `man lore` works.

A search bar sits on top, the list below it, and the keys you can press are
always shown on the bottom line:

- ↑ and ↓ move through the commands (the letter headers are skipped).
- → or Enter opens the selected command's page beside the list: what it
  is, its package and version (and whether an update is known), how to use
  it with its most common uses, links (its own site, and Stack Overflow,
  GeeksforGeeks and Google searches; Ctrl+click opens one in the browser,
  in Konsole and most terminals), then its manual page, all laid out to fit
  the space left; the list narrows to a tenth of the
  width, and names too long for it end in `...`. ↑ and ↓ scroll the page a
  line, PgUp and PgDn a screen; `/` searches other commands with the page still
  beside the list, and Enter opens the one you pick in its place; ← or Esc close it, and the arrows move
  through the commands again. A command with no page shows the likely reasons why.
- `/` opens the search bar; typing keeps the names that contain what you
  typed, ignoring case, ranked best first (the exact name always on top,
  then names starting with it), and from three characters on also
  searches what the pages say: "compress folder" finds tar and zip after
  any name matches, each showing the page line that matched (`cut
  [Cut out fields from stdin or files.] [text]`). With no search, the list
  is A to Z; start with `@` to keep only the commands that
  have a lore page (`@gr`). Esc leaves the search
  bar but keeps its text and its filter; `/` goes back to it, and Backspace
  empties it.
- `c`, on a page, copies one of its examples to the clipboard (over ssh
  too: the terminal does the copy).
- `b`, in the list or on a page, bookmarks the command (again: takes it out).
- `p`, in the list or on a page, opens the colour palettes.
- Esc, when not searching or reading, quits. Ctrl+C quits at any time.

## Building

```bash
cli/make     # cargo fmt, clippy, test, build, build --release, then copies app/lore
cli/clean    # removes all of target/ and the page drafts
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

Most usage examples in `content/` are adapted from
[tldr-pages](https://github.com/tldr-pages/tldr), under
[CC BY 4.0](https://creativecommons.org/licenses/by/4.0/); see
[content/README.md](../content/README.md) for what was changed.
