# lore: developer documentation

How lore is built and where things are, for anyone working on the code.

| Looking for… | Read |
| --- | --- |
| What lore is and how to use it | [`.github/README.md`](../.github/README.md) |
| The full manual | [`man/lore.1`](../man/lore.1) (`man -l man/lore.1`) |
| What changed in each version | [`CHANGELOG.md`](../CHANGELOG.md) |
| A version in depth: contents, checks, open issues | [`versions/`](versions/) |
| Writing pages, recipes, palettes | [`content/`](../content/README.md), [`recipes/`](../recipes/README.md), [`palettes/`](../palettes/README.md) |
| How code changes are expected to look | [`CONTRIBUTING.md`](../.github/CONTRIBUTING.md) |

**This page:** [Names](#names) · [Layout](#layout) ·
[How it runs](#how-it-runs) · [How a page is made](#how-a-page-is-made) ·
[Search](#search) · [What lore keeps](#what-lore-keeps) ·
[Rules](#rules) · [Build and check](#build-and-check) ·
[Releases](#releases) · [Versions](#versions) · [Licences](#licences)

---

## Names

| | |
| --- | --- |
| The app, its binary, its `.deb` and `.rpm` | `lore` |
| The Rust crate (crates.io) | `lore-tui` (`Cargo.toml`'s `name`, with `[[bin]] name = "lore"`) |
| The Arch (AUR) package | `lore-tui`: `lore` there is another program |

---

## Layout

```text
lore/
├── src/
│   ├── main.rs             calls lore_tui::run()
│   ├── lib.rs              run(): arguments, the terminal, the event loop, the settings changes
│   ├── app.rs              the state: list, search, page, modes, settings, jumper
│   ├── compile_config.rs   every hardcoded value and text (see Rules)
│   ├── settings.rs         the settings file and the kept lists
│   ├── commands/           finding the commands
│   │   ├── mod.rs          Command, Kind, collect(), merge()
│   │   ├── path.rs         executables in $PATH and the program folders
│   │   ├── shell.rs        the user's shell, its builtins and keywords
│   │   ├── aliases.rs      aliases, read as text from the startup files
│   │   └── functions.rs    fish functions
│   ├── info/               what a page says
│   │   ├── beacon.rs       the beacon topic's numbers: popcon and Homebrew, ranked
│   │   ├── content.rs      the written pages: built in, folders, translations
│   │   ├── page.rs         the whole page as styled lines (and the INTERNET section)
│   │   ├── package.rs      package and version from dpkg/apt, rpm, pacman, Homebrew
│   │   ├── links.rs        the search links every page gets
│   │   ├── check.rs        `lore check`: the check of pages lore did not write
│   │   ├── danger.rs       what can destroy data or stop the system
│   │   ├── explain.rs      `lore explain LINE`
│   │   ├── recipes.rs      the recipes as list entries (Kind::Recipe)
│   │   ├── tldr.rs         a tldr-pages page in lore's format
│   │   └── updates.rs      `lore --update-pages`: the weekly bundle, else tldr's zips
│   ├── interface/          what the user sees and types
│   │   ├── args.rs         the command line
│   │   ├── complete.rs     Tab completion's answers (`lore --complete`)
│   │   ├── input.rs        keys to actions, per mode
│   │   ├── ui.rs           drawing
│   │   ├── palette.rs      the palettes and their styles
│   │   ├── search.rs       name ranking, search by content, snippets
│   │   ├── rows.rs         the list's rows
│   │   ├── section.rs      a topic's sections: # and A–Z
│   │   └── paths.rs        `~/` and `*` in configured paths
│   └── utils/
│       ├── clipboard.rs    copying through the terminal (OSC 52)
│       ├── history.rs      uses per command, from the shell's history
│       ├── internet.rs     the connection dot and cht.sh's cheat sheets
│       ├── manual.rs       running man and whatis, cleaning their output
│       ├── text.rs         fitting text to columns
│       └── zip.rs          the files in a zip archive (stored or deflated)
├── build.rs                packs content/ into the binary (zstd); embeds and checks palettes/ and recipes/
├── content/                9,109 pages, and 32,226 translations in i18n/<language>/ (41 languages)
├── recipes/                100 task pages
├── palettes/               15 colour palettes
├── beacon/beacon.tsv       the beacon's built-in table (BEACON_SNAPSHOT)
├── completions/            bash, zsh and fish scripts (they ask `lore --complete`)
├── man/lore.1              lore's own manual
├── tests/                  every test (32 files), through src/lib.rs
├── examples/commands.rs    prints every command lore lists, for scripts/content
├── scripts/content/        the scripts that made content/, and bundle.py (the weekly pages)
├── scripts/beacon/         snapshot.py: the beacon's table, from popcon and Homebrew
├── scripts/release/        the release tarball and notes
├── packaging/              the AUR build file, the Homebrew formula, update.sh
├── cli/make, cli/clean     the build gate, and cleaning
├── .github/workflows/      ci, docs, scripts, release, content (weekly data)
└── app/                    where cli/make copies the release binary (ignored)
```

---

## How it runs

```text
main.rs → lib.rs::run()
            ├─ a command-line job (--print, explain, check, --complete…): prints, exits
            └─ the app:
                 commands::collect() + recipes  →  App (app.rs)
                 loop:  ui::draw(App)  →  input::next_action(mode)  →  lib.rs applies it to App
```

The app is always in one **mode**, and each mode has its own keys
(`interface/input.rs`):

| Mode | What is on screen | Keys |
| --- | --- | --- |
| Browse | The topic bar and its list | ↑↓, ←→ (topic), Enter, Tab / Shift+Tab (letter), `/`, `b`, `+`, Esc quits |
| Search | Typing in the search bar | text, Backspace, ↑↓, Enter, Esc |
| Info | A page beside the list | ↑↓ PgUp PgDn, Tab / Shift+Tab (section), `/`, `c`, `b`, ← / Esc |
| Copy | The page's examples, over it | ↑↓, Enter copies, Esc |
| About | The `+` screen and its settings | ↑↓ choose, Enter changes, `p`, + / ← / Esc |
| Palette | The palette picker, over `+` | ↑↓ try, Enter keeps, Esc cancels |

Ctrl+C quits in every mode. The background (the internet) never changes
the state itself: `next_action` checks every 250 ms whether something
arrived, and asks for a redraw.

---

## How a page is made

1. **The list.** `commands::collect()` scans the program folders
   (`PROGRAM_DIRS`, `$PATH`, macOS's `/etc/paths`), adds the shell's
   builtins and keywords, the user's aliases, and the subcommands that
   have a page when their program is installed (`git commit`). Each name
   is kept once, the way bash would resolve it.
2. **The written page.** `info::content::find` looks for `<name>.md`:
   1. in the reader's own folder (`USER_CONTENT_DIRS`), which may replace
      anything;
   2. in the pages built into the binary;
   3. in the pages `lore --update-pages` fetched (`UPDATED_CONTENT_DIRS`),
      then the folders other programs install pages in
      (`SYSTEM_CONTENT_DIRS`), both only for a command lore has no page for.

   A page from a folder must pass `info::check`, or it is skipped. A
   subcommand's spaces become dashes in the file name (`git-commit.md`);
   characters Windows forbids become `%XX`. In the reader's language
   (`LC_ALL`, `LC_MESSAGES`, `LANG`), a translation's summary and examples
   replace the English ones.
3. **The facts.** `info::page::facts` adds what the machine knows: the
   `whatis` line, the program's file, its package and version. Looked up
   once per command.
4. **The layout.** `info::page::render` lays it out for the panel's width:

   > name · categories · summary · danger warning · VERSION · HOW TO USE ·
   > RELATED · SUBCOMMANDS · NOTES · FILES · LINKS · *INTERNET* · MANUAL

   It runs again on every resize, without looking the facts up again.
   **INTERNET** is added at each draw by `ui::draw_page` (just before
   MANUAL), because its answer arrives in the background. A **recipe's**
   page is STEPS · USES · NOTES instead.

---

## Search

- **No search:** the topic's list (`App::topic`, `Topic` in app.rs):
  commands and recipes A to Z under their letters, most used / recent /
  bookmarks in their own order, the beacon under its own headings
  (`App::beacon_rows`: `Row::Title`, which Tab jumps between like letters). The bar shows only topics with anything
  (`App::topics`); one that empties gives way to commands. A new kind of
  lore is a new `Topic`.
- **A search** looks in every topic, ranked best first, with no headers:
  1. names: exact, then starting with the text, then a word inside the
     name starting with it, then anywhere;
  2. pages that match by content (from three characters): categories count
     most, then the summary, then the examples' descriptions, then the
     examples. Words match at word starts, `SYNONYMS` and plurals count as
     the same word, and a command named by one of the words goes first.
- Ties go to the commands your history runs most.
- Each result shows the page line that matched, then its categories.
- `@` first keeps only commands with a page.

---

## What lore keeps

| File | Written by | Holds |
| --- | --- | --- |
| `$XDG_CONFIG_HOME/lore/config` (`~/.config/lore/config`) | the `+` settings (`lib.rs::change_setting`) | `palette`, `internet`, `history` |
| `$XDG_CONFIG_HOME/lore/palettes/` | the user | their own palettes |
| `$XDG_DATA_HOME/lore/` (`~/.local/share/lore/`) | `b`, opening a page | `bookmarks`, `recent` |
| `$XDG_CACHE_HOME/lore/internet/` (`~/.cache/lore/internet/`) | `utils::internet` | cheat sheets, a day |
| `$XDG_CACHE_HOME/lore/beacon.tsv` | `info::beacon::refresh` (Internet on) | the newest beacon table, a week |
| `~/.local/share/lore/updates/` | `lore --update-pages` (`info::updates`) | pages and translations lore lacks |

---

## Rules

- **Every hardcoded value and fixed text goes in `src/compile_config.rs`**,
  and only there: no `const` or `static` anywhere else (CI checks it). That
  covers paths, builtins and keywords, every text the user reads, colours
  and styles, widths, search weights, synonyms, danger rules, links and
  update commands. Only the format of what lore *reads* (page sections,
  alias syntax, package managers' output) stays next to its parser.
- **A missing path or file is skipped**, never an error.
- **lore never runs the commands it lists.** Only `man`, `whatis` and the
  package managers' read-only queries, with a plain argument, never through
  a shell.
- **Only `https://` links** are shown, and lore never opens one itself.
- **Text is measured in columns**, not characters (`utils::text`): Chinese,
  Japanese and Korean characters take two.
- **Tests live in `tests/`**, reach the code through `src/lib.rs`, and are
  named as sentences that state the rule they defend.
- **Every new feature** comes with tests, a CHANGELOG line and docs.
- **Comments explain why**, short, for someone new to the code.

---

## Build and check

```bash
cli/make      # cargo fmt, clippy (warnings fail), test, build, build --release → app/lore
cli/clean     # removes target/, dist/ and .content-draft/
```

Rust 1.88 or newer (ratatui 0.30.2 needs it), edition 2024.

**On GitHub**, on pushes to `main` and pull requests that touch what each
one checks:

| Workflow | Checks |
| --- | --- |
| `ci.yml` | What `cli/make` does, the constants rule, and a run of the release binary (`--print`, `explain`, `--complete`) |
| `docs.yml` | Links between the repository's own docs (offline), and `man/lore.1` through `groff -ww` |
| `scripts.yml` | shellcheck on the shell scripts, and that the Python scripts parse |

**The screen** is not covered by the tests. Check it by hand, in a real
terminal or with tmux:

```bash
tmux new-session -d -x 100 -y 30 app/lore
tmux send-keys / t a r
tmux capture-pane -p
```

lore needs a terminal: started from a pipe, it says so and exits with 1.

---

## Releases

1. Make sure `Cargo.toml`'s version is the one to release, and the
   CHANGELOG has its changes (under its own `## [x.y.z]` heading, or still
   under `## [Unreleased]`).
2. Tag and push: `git tag vX.Y.Z && git push origin vX.Y.Z`.
3. `release.yml` checks that the tag matches `Cargo.toml` and runs the
   tests, then builds:
   - static musl binaries for x86_64 and aarch64 Linux, each with a `.deb`
     and an `.rpm` (`[package.metadata.deb]` and `generate-rpm` in
     `Cargo.toml`);
   - macOS binaries for Apple Silicon and Intel (not signed);
   - a GitHub release with all of them, `SHA256SUMS`, and the notes from
     `scripts/release/notes.sh`.
4. **The weekly data** is not part of a release: `content.yml` runs each
   Monday (or by hand) and replaces the two assets of the pre-release
   tagged `content`: `pages.zst` (`scripts/content/bundle.py`, for
   `lore --update-pages`) and `beacon.tsv` (`scripts/beacon/snapshot.py`).
   lore reads them at fixed URLs (`UPDATE_BUNDLE_URL`, `BEACON_URL`); when
   they cannot be had, it goes to tldr-pages, popcon and Homebrew itself.
   To refresh the built-in table before a release, run
   `python3 scripts/beacon/snapshot.py`.
5. **AUR and Homebrew,** by hand: `packaging/update.sh X.Y.Z` puts the new
   version and checksums into `packaging/aur/PKGBUILD` and
   `packaging/homebrew/lore.rb`, then prints what to push where.

---

## Versions

From 0.1.0 on, every commit raises the version in `Cargo.toml`, and its
subject starts with it: `v0.1.6 - root: …`. A version users will notice
gets a record in [`versions/`](versions/), named with dashes (`0-1-6.md`),
and a section in the CHANGELOG.

---

## Licences

The code is MIT ([`LICENSE`](../LICENSE)). Most usage examples, and every
translation, are adapted from [tldr-pages](https://github.com/tldr-pages/tldr)
under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/); see
[`content/README.md`](../content/README.md) for the commit used and what was
changed.
