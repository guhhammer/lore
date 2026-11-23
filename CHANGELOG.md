# Changelog

Every notable change to this repository, newest first. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Versioning is
[semantic](https://semver.org/spec/v2.0.0.html) in shape, but everything below
1.0.0 is prototype work: nothing here is stable yet.

Up to 0.1.0, commit subjects carried a running count (`v0.0.9` was the
ninth commit before the first version). From 0.1.0 on, every commit raises
the version in `Cargo.toml`, and its subject starts with that version
(`v0.1.1 - …`). Each version's record is in
[`.documentation/versions/`](.documentation/versions/).

## [Unreleased]

### Changed

- The built-in pages are packed with zstd at build time and unpacked once
  when lore starts (about 50 ms), and the release build optimises the whole
  program as one unit (thin LTO, one codegen unit): the binary goes from
  27.0 MB to 8.2 MB.

## [0.1.0] - 2026-11-23

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
- A command's manual page: → or Enter opens it beside the list, laid out by
  `man` at the panel's width and laid out again when the terminal is
  resized; ↑/↓ scroll it a line, PgUp/PgDn a screen, ← or Esc close it.
  With no page, the likely reasons are shown. While a page is open the list
  takes a tenth of the width; names too long for any row end in `...`.
- Command pages: the page beside the list now starts with a summary, a
  VERSION section (package, installed and newest known version, how to
  update, where the program is; from dpkg/apt, rpm, pacman or Homebrew's
  folders, offline), HOW TO USE with the most common uses, NOTES, and
  LINKS (the page's own, plus Stack Overflow, GeeksforGeeks and Google
  searches, opened by the terminal's Ctrl+click), with a note under them
  that links must be checked and that lore never asks anyone to log in.
  The manual page follows. Pages are Markdown files in `content/`, built
  into the binary (14 so far); `~/.local/share/lore/content` can add or
  replace one. Without a page, the summary comes from `whatis`.
- Pages for about 8,800 commands in `content/` (every tldr-pages page for
  Linux, macOS and common commands, adapted under CC BY 4.0, plus pages
  written from local manual pages and by hand), all in one structure.
- Searches are ranked, best first, instead of A to Z: the exact name on
  top, then names starting with the text, then at a word start inside a
  name, then anywhere, then matches by page content. The A-Z sections stay
  for the list with no search.
- Each search result shows the page line that matched, cut around the
  match, between its name and its categories; content words match at word
  starts only, plurals also find singulars, and a command named by one of
  the words ranks first among content matches.
- Search by content: from three characters on, the search also looks
  through the pages' categories, summaries and examples, best match first
  under "found in pages"; common words count as their equivalents (folder
  as directory).
- Categories on every page and after each name in the list; `danger` (in
  red) for commands that can destroy data or stop the system, a warning on
  their page, and `!` on every risky example.
- RELATED on each page: the page's See also (from tldr) and the man page's
  SEE ALSO.
- Pages in 41 languages from tldr-pages' translations, picked from
  `LC_ALL`, `LC_MESSAGES` or `LANG`.
- `/` on a page searches other commands while the page stays on screen;
  Enter or → opens the chosen one in its place.
- Subcommands: about 1,900 pages such as `git commit` and `docker run`,
  listed under their program when it is installed, named on its page
  (SUBCOMMANDS), and opened with `lore git commit`.
- Pages for Windows, BSD, SunOS and Android commands (from tldr-pages),
  opened with `lore NAME`, each saying which system it is for.
- Every page links more places people look for help: Unix & Linux Stack
  Exchange, Ask Ubuntu, Super User, ArchWiki, explainshell, cheat.sh,
  Debian man pages, man7.org and SS64, besides Stack Overflow,
  GeeksforGeeks and Google.
- `lore NAME` opens NAME's page straight away, like `man NAME`; closing it
  quits. A name with nothing to show is reported in the shell, exit 1.
  `lore --help` and `lore --version`.
- `man/lore.1`, lore's own manual page, and a lore page for lore itself.
- `@` at the start of a search keeps only the commands that have a lore
  page.
- Leaving the search bar (Esc) keeps its text and filter; `/` returns to it.
- `scripts/content/`: the scripts that make the pages from tldr-pages and
  the local man pages, in seven steps (their README), and
  `examples/commands.rs` to list the commands they start from. Every
  `mkfs.*` page is now marked `danger`.
- `cli/make` (fmt, clippy, test, build, release, copy to `app/`) and
  `cli/clean` (all of `target/`, and the page drafts in `.content-draft/`).
- The MIT licence, and the repository's pages: the README, `SECURITY.md`,
  `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, issue and pull-request templates,
  and the workflows, which only pass for now.
