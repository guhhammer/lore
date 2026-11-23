# lore: project documentation

Notes for whoever works on lore: how it is built, where things are, and what
each version contained. The user-facing page is
[`.github/README.md`](../.github/README.md); lore's own manual is
[`man/lore.1`](../man/lore.1) (`man -l man/lore.1`).

| Page | What it holds |
| --- | --- |
| this file | the map: layout, how a page is made, rules, how to build |
| [`versions/`](versions/) | one record per version: what it contains, how it was checked, what is open |
| [`../CHANGELOG.md`](../CHANGELOG.md) | the short list of changes per version |
| [`../content/README.md`](../content/README.md) | the page format, for people who write pages |

## What lore is

A terminal program that lists every command you can type on this machine,
A to Z, and shows a page for each: what it does, its package and version,
its most common uses, related commands, links and its manual. Offline: the
pages are built into the binary. Linux and macOS; bash, zsh and fish.

## Layout

```text
lore/
├── src/
│   ├── main.rs             calls lore_tui::run()
│   ├── lib.rs              run(): arguments, the terminal, the event loop
│   ├── app.rs              the state: list, search, page, modes
│   ├── compile_config.rs   every hardcoded value and text (see "Rules")
│   ├── commands/           finding the commands
│   │   ├── mod.rs          Command, Kind, collect(), merge()
│   │   ├── path.rs         executables in $PATH and the program folders
│   │   ├── shell.rs        the user's shell, its builtins and keywords
│   │   ├── aliases.rs      aliases, read as text from the startup files
│   │   └── functions.rs    fish functions
│   ├── info/               the page beside the list
│   │   ├── content.rs      the written pages: built in, folders, translations
│   │   ├── page.rs         the whole page as styled lines
│   │   ├── package.rs      package and version from dpkg/apt, rpm, pacman, Homebrew
│   │   ├── links.rs        the search links
│   │   └── danger.rs       what can destroy data or stop the system
│   ├── interface/          what the user sees and types
│   │   ├── args.rs         `lore`, `lore NAME`, --help, --version
│   │   ├── input.rs        keys to actions, per mode
│   │   ├── ui.rs           drawing
│   │   ├── search.rs       name ranking, search by content, snippets
│   │   ├── rows.rs         the list's rows
│   │   ├── section.rs      `#` and A-Z
│   │   └── paths.rs        `~/` and `*` in configured paths
│   └── utils/
│       ├── manual.rs       running man and whatis, cleaning their output
│       └── text.rs         fitting text to columns (`...`, wrapping)
├── build.rs                builds content/ into the binary
├── content/                the pages (~9,100) and i18n/<language>/ (~32,200)
├── man/lore.1              lore's own manual
├── scripts/content/        the scripts that make content/ (their README has the order)
├── examples/commands.rs    prints every command lore lists, for scripts/content
├── tests/                  every test, through src/lib.rs
├── cli/make, cli/clean     the build gate and cleaning
└── app/                    where cli/make copies the release binary (ignored)
```

## How a command's page is made

1. **The list.** `commands::collect()` scans the program folders
   (`PROGRAM_DIRS`, `$PATH`, macOS's `/etc/paths`), adds the shell's
   builtins and keywords, the user's aliases, and the subcommands that have
   a page when their program is installed (`git commit`), then keeps each
   name once, the way bash would resolve it.
2. **The written page.** `info::content::find` looks for `<name>.md` in the
   `CONTENT_DIRS` folders, then in the pages built into the binary. A
   subcommand's spaces become dashes in the file name (`git-commit.md`);
   characters Windows forbids become `%XX` (`:` is `%3A.md`). In the
   reader's language (`LC_ALL`, `LC_MESSAGES`, `LANG`), a translation's
   summary and examples replace the English ones.
3. **The facts.** `info::page::facts` adds what the machine knows: the
   `whatis` line, the program's file, its package and version. These are
   looked up once per command.
4. **The layout.** `info::page::render` lays it all out for the panel's
   width, in this order: name, categories, summary, danger warning,
   VERSION, HOW TO USE, RELATED, SUBCOMMANDS, NOTES, LINKS (and the note
   under them), MANUAL (`man` at the panel's width). It runs again on every
   resize; the facts are not looked up again.

## Search

- With no search, the list is A to Z under its letters.
- With a search, it is ranked best first, with no headers. Names come first:
  exact, then starting with the text, then at a word start inside the name,
  then anywhere. Then pages that match by content (from three characters):
  categories count most, then summary, then the examples' descriptions,
  then the examples. Words match at word starts; `SYNONYMS` and plurals
  count as the same word; a command named by one of the words goes first.
- Each result shows the page line that matched, then its categories.
- `@` first keeps only commands with a page.

## Rules

- **Every hardcoded value and fixed text goes in `src/compile_config.rs`**,
  and only there: no `const` or `static` anywhere else. That covers paths,
  builtins and keywords, every text the user reads (help, errors, headings,
  key bar, hints), colours and styles, widths and indents, search weights
  and limits, synonyms, danger rules, search sites and update commands.
  What stays in the code is the format of what lore reads (page sections,
  alias syntax, the package managers' output and flags), next to its parser.
- **Nothing is ever an error for a missing path or file**: it is skipped.
- **lore never runs the commands it lists.** It runs only `man`, `whatis`
  and the package managers' read-only queries, with the name or path as a
  plain argument, never through a shell.
- **Only `https://` links** are shown, and lore never opens one itself.
  Every page ends its links with the same warning.
- **Text is measured in columns**, not characters (`utils::text`): Chinese,
  Japanese and Korean characters take two.
- **Tests live in `tests/`**, reach the code through `src/lib.rs`, and are
  named as sentences that state the rule they defend.
- **Comments explain why**, short, for someone new to the code.

## Building and checking

```bash
cli/make      # cargo fmt, clippy (warnings fail), test, build, build --release, copy to app/lore
cli/clean     # removes all of target/ and .content-draft/ (drafts and the tldr download)
```

The drawing and the keys are not covered by the tests. Check them by hand
in a real terminal, or with tmux:

```bash
tmux new-session -d -x 100 -y 30 app/lore
tmux send-keys / t a r
tmux capture-pane -p
```

lore needs a terminal. Started from a pipe it says so and exits with 1, so
never run it inside another tool's output.

## Versions

From 0.1.0 on, every commit raises the version in `Cargo.toml`, and its
subject starts with that version. A version that adds or changes something
users see gets a record in [`versions/`](versions/), named after the
version with dashes (`0-1-0.md`), and a section in the CHANGELOG.

## Content sources and licences

The code is MIT ([`LICENSE`](../LICENSE)). Most usage examples, and every
translation, are adapted from [tldr-pages](https://github.com/tldr-pages/tldr)
under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/); see
[`content/README.md`](../content/README.md) for the commit used and what was
changed.
