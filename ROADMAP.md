# Roadmap

Where lore has been, where it goes next, and what it will never do. Each
change in detail is in [CHANGELOG.md](CHANGELOG.md); each release's record
is in [`.documentation/versions/`](.documentation/versions/).

## Done

### 0.1.0: the list and its pages (2026-11-23)

- Every command you can type, A to Z: programs, shell builtins and
  keywords, aliases, in bash, zsh and fish, on Linux and macOS.
- A page for each: what it does, examples, categories (`danger` in red),
  RELATED, the man page beside the list; about 8,800 pages from tldr-pages
  and the man pages, and subcommands such as `git commit`.
- A ranked search, by name and by what the pages say, with the line that
  matched; pages in 41 languages from tldr-pages' translations.

### 0.1.6: the first release (2026-11-24)

- Static Linux binaries (x86_64 and ARM), `.deb` and `.rpm` packages, macOS
  binaries, each with the man page and Tab completion for bash, zsh, fish.
- VERSION on every page (package, installed, newest, how to update), FILES,
  LINKS, risky examples marked `!`.
- `lore explain LINE`, `lore --print NAME`, `lore check FILE`, pages that
  other programs ship, colour palettes as files, a settings file.
- Recipes: 100 task pages; bookmarks and recent pages; the INTERNET section
  from cht.sh, cached, and off with one setting.

### 0.3.3: topics, docs and the agent plugin (2026-11-24)

- Topics under the search bar: commands, recipes, languages (201
  learnxinyminutes pages), docs (DevDocs' 850 sets, read offline), tools
  (46 cheat sheets), beacon (what people use most and what is rising), most
  used, recent, bookmarks.
- Quick keys (`g`/`G`, `n`/`N`, `y`, `o`, `1`–`9`, `B`, `r`, `?`), 34
  palettes (11 for colour blindness and low vision), small windows.
- `lore search WORDS` and `lore --update-pages`; the weekly `content`
  workflow.
- A Claude Code plugin: it installs lore by itself and its skill answers
  command-line questions from lore instead of the web.

### 0.3.4 to 0.4.0: since that release

- The AUR and Homebrew files point at the v0.3.3 release.
- The skill searches lore with key words, not a sentence.
- The launch video, and `lore-2.svg`, which plays it in the README; a README
  at the root for Claude Code's plugin directory.
- The interface in your language: the whole screen and the recipes' names
  follow Pages in, in 12 languages (`i18n/`); what lore prints for scripts
  stays in English.
- A box over Chinese, Japanese or Korean text keeps its border.
- Windows users: how to run lore inside WSL, in the README.

## Next

- Make the repository public, so the plugin, the downloads and the
  Homebrew tap work for everyone.
- Publish the crate `lore-tui` on crates.io (`cargo install lore-tui`).
- Publish the AUR package `lore-tui`.
- More languages for the interface: a new folder in `i18n/` (format in
  [i18n/README.md](i18n/README.md)).

## Never

- **A native Windows version.** lore is a program dedicated to Linux. On
  Windows it runs inside WSL: see the README's
  [Windows (WSL)](.github/README.md#windows-wsl) section.
