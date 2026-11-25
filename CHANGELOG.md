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

## [0.3.3] - 2026-11-24

### Added

- **A new demo that plays in the README:** `.github/media/lore.svg`, an
  animated SVG of the app itself (search, a page, a tool's cheat sheet, the
  keys, the languages), 310 KB, in place of the 4.6 MB video GitHub could
  not play. `scripts/demo/demo.py` makes it again from `app/lore`, in an
  empty home folder.

- **`lore search WORDS`:** the search bar's results as plain text, the best
  first (at most 25): name, the line that matched, categories, tab apart.
  Exits 1 when nothing matches. Tab completion offers it.
- **A plugin with a skill:** `claude-plugin/skills/lore/SKILL.md` answers
  questions about command-line tools from lore (`lore search`, `lore -p`,
  `lore explain`, no paths); `.claude-plugin/marketplace.json` makes the
  repository installable as a plugin. Installing it is all it takes: at
  each session start `claude-plugin/scripts/setup.sh` installs lore when it
  is missing or too old (the latest release for the system, checked against
  `SHA256SUMS`, into `~/.local/bin`, no sudo) and gives the skill its path;
  `/lore:setup` does it by hand, `--update` gets the latest. See the README,
  "Use lore from an agent".

- **Quick keys:** `g`/`G` go to the top or the bottom of the list or the
  page; `n`/`N` on a page jump to the next or previous line with the
  search's text; `y` copies a page's first example at once (its name when
  it has none; nothing on a language's page); `o` shows a page's first link
  alone in a box to Ctrl+click; `1`–`9` go to a topic, `B` to the
  bookmarks, `r` to the recent pages; `?` shows every key; Ctrl+L draws the
  screen again.

- **Palettes for colour blindness and low vision:** `modus-vivendi`,
  `modus-operandi` and their `-deuteranopia` and `-tritanopia` versions
  (every colour at least 7:1, WCAG AAA), `github-dark-colorblind`,
  `github-light-colorblind`, `github-dark-high-contrast`,
  `github-light-high-contrast`, and `okabe-ito` (the colour set made to
  stay apart in every kind of colour blindness). 33 palettes in all.
- **8 more palettes:** `monokai`, `github-dark`, `github-light`,
  `everforest-dark`, `everforest-light`, `kanagawa`, `ayu-dark` and
  `ayu-light` (23 in all).
- **Small windows:** the key bar wraps onto a second row (never more) when
  one is too narrow, the topics bar slides to keep the chosen topic in
  sight, and a window under 50×14 says its size and what lore needs.
- **Copy a doc's code:** `c` on a doc's page lists its code blocks and
  copies only the code, not the panel around it. On a language's page `c`
  copies nothing and says so: its code is there to be typed by hand.
- **The `+` screen scrolls** on a window too small for it: PgUp, PgDn, `w`
  and `s` scroll it, and choosing a setting with ↑↓ scrolls to it.
- **`scripts/content/wikipedia.py`:** adds a Wikipedia link, last under
  `## Links`, to a command page whose program has an article about software
  on English Wikipedia. It has run: 1,571 pages link their program's
  article (a subcommand its parent's); 59 matches that named another
  article were taken out by hand.
- **The docs topic:** every documentation set DevDocs offers (850, for
  languages, frameworks and tools), the 50 most asked about on Stack
  Overflow first, then every set A to Z. `d` downloads a set in the
  background to read offline (`~/.local/share/lore/docs/`), `x` deletes it.
  Enter on a downloaded set lists its pages by type; `/` searches them (or
  the sets); an entry opens at its own part, its HTML shown as text with
  headings, lists and code. Esc goes back to the sets. The catalogue is
  built in (`devdocs/catalog.tsv`, from `scripts/docs/catalog.py`).
- **The tools topic:** a cheat sheet per tool, from `tools/` (built in,
  format in `tools/README.md`): the everyday tasks people look up for
  networking (ip, ss, dig, curl, tcpdump), language environments (python
  venv, pip, uv, nvm, npm, pnpm, cargo, rustup), git and gh, databases
  (MySQL, PostgreSQL, SQLite, Redis, MongoDB), containers (docker, docker
  compose, podman, kubectl, helm), tmux, vim, shell scripting, servers
  (systemd, cron, nginx, ufw, firewalld, certbot), keys and access (ssh
  keys, gpg, pass, openssl), cloud (aws, gcloud, az, terraform, ansible)
  and files (rsync, tar and zip, find grep sed awk). Each page's tasks are
  in groups the jumper (Tab) stops at; `c` copies one. A tool named like a
  command here is listed as `NAME (tool)`.
- **Reset all** on the `+` screen: Enter, then Enter again, puts every
  setting back to its default and erases what lore kept (bookmarks, recent
  pages, downloaded docs and pages, the caches). Your own pages and
  palettes stay.
- **Follow** on the `+` screen: the author's GitHub, LinkedIn and X.
- The search reads the pages in the language set in Pages in (or `LANG`),
  English where there is no translation, and again when Pages in changes.

- **PgUp and PgDn in the list** (and the search's results): 20 rows at a
  time, as on a page.
- **Fast scroll:** `w` and `s` move up and down by a number of lines, in
  the list and on a page. Set on the `+` screen (Fast scroll: 5, 10, 15, 20
  or 30; 10 at first), saved as `scroll = 10`.
- **Pages in**, on the `+` screen: the language lore shows pages in
  (`system` follows `LANG`), from a picker of every language there is a
  translation in; saved as `reading = pt_BR`.
- **Favourite**, on the `+` screen: a programming language shown first in
  the languages topic, under `favourite`; saved as `favourite = Python`.
- In a search, the `language` category of a languages page is in the
  palette's third colour, so languages stand out among commands.

- **The languages topic:** the pages of
  [Learn X in Y minutes](https://learnxinyminutes.com), 201 programming
  languages, tools and formats, with the site's 678 translations in 37
  languages (picked from `LANG`, like the command pages). Each is a reminder
  of how a language works, read from top to bottom: an introduction, then
  one commented file, shown the way `bat` shows a file, with line numbers
  and colours for comments, strings, numbers and keywords, wrapped under
  its number. Tab and Shift+Tab jump between the file's parts (its
  `####` banners). The pages are copied unchanged into `languages/`
  (`scripts/languages/sync.py`), licensed CC BY-SA 3.0 and credited on each
  page. A language named like a command here gets ` (language)`.

- **The beacon topic:** what people use most and what is rising, between
  recipes and most used. It lists the commands trending this month, the
  most used ones, then the most used per category (for archives, for
  network…), each with its rank, its rise and whether it is installed
  here. A command lore has a page for is listed even when it is not
  installed, to read about before installing it. The numbers are public:
  Debian's popularity contest and Homebrew's installs asked for by name. A
  table of them is built in (`beacon/beacon.tsv`, from
  `scripts/beacon/snapshot.py`), so the topic works offline. With Internet
  on, a newer one is fetched at most once a week for the next start: lore's
  own copy first, else from the two sources.
- **`lore --update-pages`:** downloads newer pages, only when you ask. First
  lore's weekly pages (`pages.zst`, every tldr page in lore's format, from
  `scripts/content/bundle.py`), else tldr-pages' own zip files, converted
  on your machine. It keeps only pages and translations lore lacks, each
  passing `lore check`, in `~/.local/share/lore/updates/`, read after the
  built-in pages: they never change a page lore has.
- **The `content` workflow** publishes `pages.zst` and `beacon.tsv` each
  week as the assets of a pre-release tagged `content`, so the README's
  latest-release link stays on lore's newest version.

### Fixed

- A translated tldr page whose summary holds a link no longer loses its
  summary (the link was taken for the "more information" line).
- `lore --help` described old keys (p in the list, → to open a page).
- The release workflow's macOS builds ask for `macos-latest`: GitHub no
  longer has `macos-14`, so v0.1.6's release never finished.
- The crate's package now holds `recipes/`, `palettes/` and `completions/`,
  which the build needs (`cargo package` verifies it builds alone).

### Changed

- **Source layout:** `src/app.rs` is now the folder `src/app/` (state, types,
  topics, the page on show, the about screen) and `src/compile_config.rs`
  is `src/configs/`, one file per part of lore; every value is reached as
  `crate::configs::NAME`.
- **Topics:** a bar under the logo and the search bar holds lore's topics,
  each a list of its own: commands, recipes, and your most used, recent
  and bookmarks once they have anything. ← and → switch topics; a search
  still looks in every topic, and `lore NAME` opens on NAME's topic. The
  groups that used to head the list are these topics now, and most used
  lists 50 commands, not 10.
- **Keys:** Enter opens a page from the list (→ only from the search bar);
  Tab and Shift+Tab jump to the next or previous letter, or section on a
  page, in place of Space. The key bar says `↑↓←→ Move`, `+ About` and
  `Esc/^C Quit`.

- `Cargo.toml` names the homepage, https://lore.guhhammer.dev, and better
  crates.io keywords and categories.

## [0.1.6] - 2026-11-24

The first release published on GitHub: a static binary for any Linux
(x86_64 and ARM), `.deb` and `.rpm` packages, and macOS binaries for Apple
Silicon and Intel, each with the man page and the shell completions.
Everything since 0.1.0 is in it.

### Added

- **Palettes for colour blindness and low vision:** `modus-vivendi`,
  `modus-operandi` and their `-deuteranopia` and `-tritanopia` versions
  (every colour at least 7:1, WCAG AAA), `github-dark-colorblind`,
  `github-light-colorblind`, `github-dark-high-contrast`,
  `github-light-high-contrast`, and `okabe-ito` (the colour set made to
  stay apart in every kind of colour blindness). 33 palettes in all.
- **8 more palettes:** `monokai`, `github-dark`, `github-light`,
  `everforest-dark`, `everforest-light`, `kanagawa`, `ayu-dark` and
  `ayu-light` (23 in all).
- `packaging/`: an Arch build file (the AUR package `lore-tui`, built from
  source) and a Homebrew formula (the release binaries, for a tap), with
  `packaging/update.sh VERSION` to put a release's version and checksums
  into both.

- Settings on the about screen (`+`): Colours (Enter opens the palette
  picker over it), Internet on or off and History read or not. ↑↓ choose
  one, Enter changes it at once and saves it in the settings file. The
  internet can now be turned on and off while lore runs; turned off, the
  checking stops and the INTERNET section shows nothing fetched. `p` opens the
  palettes there only: the list and pages no longer take it.
- The jumper, Space: in the list, to the next letter or group (A, B, C, …
  and back to the top); on a page, to the next section, and back to the
  top after the last.
- `lore explain "tar -xzvf a.tgz"`: each word of a command line and what it
  means, offline: the program's summary, each flag's first sentence from
  its man page (joined short flags split, `--opt=value` read, a flag's
  value named), pipes and lists explained command by command,
  redirections, and `sudo`/`env`/`xargs` then the command they run.
- The internet: `● internet` on the search bar, green when cht.sh can be
  reached and red when not, also on the about screen. Each command's page
  gets an INTERNET section before MANUAL with that dot and the command's
  community cheat sheet from cht.sh, fetched in the background when the
  page opens and cached for a day (`~/.cache/lore/internet/`). It is shown
  as raw text behind a bar, with every control, escape and invisible
  character taken out; never turned into links, never offered to copy,
  never run; and the section reminds the reader it can be changed by anyone
  without checks. `internet = off` in the settings stops every request.
  lore makes the requests itself (ureq, rustls): the binary is about 2.5 MB
  larger.
- Pages for other programs: a program can install its own page (and
  translations, `i18n/<lang>/`) in `/usr/share/lore/content/`, used for a
  command lore has no page for; only the reader's own folder may replace a
  built-in page. `lore check NAME.md` checks such a page, and every page
  read from a folder must pass it or it is not shown: no escape sequences,
  invisible or reordering characters, HTML or script, examples that run a
  download (`curl … | sh`), wrong titles, or over 64 KB; odd links are
  warnings. Invisible and bidirectional characters are now taken out of
  every page shown.
- Shell completion for bash, zsh and fish: Tab after `lore` completes the
  names lore lists, a program's subcommands (`lore git <Tab>`), recipes
  word by word, the options, and a program's flags after `explain`; zsh
  and fish show what each one is. lore answers itself (`lore --complete`),
  so the scripts in `completions/` are three short files. The packages
  install them; `lore --completions SHELL` prints one.
- Recipes: 100 task pages that chain commands, most of them the terminal
  tasks with the most-viewed Stack Overflow questions (find files containing
  a text, change a folder's permissions, copy files to a server, send output
  and errors to a file, split a string in bash…), plus free disk space,
  what is using a port, why won't a service start and add a user, under their own `recipes` header in the
  list, found by the search, with numbered steps, the commands they use and
  notes for other systems. One Markdown file each in `recipes/`, built in
  and checked by `build.rs` (`recipes/README.md`).

- Releases: pushing a version tag (`v0.1.3`) builds lore for Linux (static
  x86_64 and ARM binaries that run on any distro, a `.deb` and an `.rpm` for
  each) and macOS (Apple Silicon and Intel), and publishes them, with
  checksums, as a GitHub release (`.github/workflows/release.yml`).
- Colour palettes: `p` in the list or on a page opens a picker of eight
  themes, most in a dark and a light variant (Catppuccin Mocha/Latte,
  Dracula, Nord, Gruvbox dark/light, Solarized dark/light, Tokyo Night/Day,
  One Dark/Light, Rosé Pine/Dawn); the arrows try one on the whole screen,
  Enter keeps it, Esc goes back. They paint their own background, and each
  theme has its own primary colour (headers, headings, the active border)
  and secondary one (categories) from its signature tones. The default,
  `terminal`, keeps the terminal's colours.
- `lore --print NAME` (`-p`): a page as plain text on standard output, with
  the same sections, for pipes and scripts; it needs no terminal.
- FILES on a page: the configuration files, logs and data a command's
  manual names (`~/.ssh/config` for ssh).
- Bookmarks (`b`) and recently opened pages, kept in `~/.local/share/lore/`,
  head the list with the search empty, with the commands your shell history
  ran most. In a search, the commands you run most come first among equal
  matches, and a dim dot marks the commands your history never ran. The
  history (bash, zsh, fish) is only read, locally; `history = off` in the
  settings file stops it.
- Palettes are files, `palettes/<name>.palette` (`key = value`, one per
  colour role), built in and checked by `build.rs`: a new palette is one
  file in a pull request (`palettes/README.md`). Your own go in
  `~/.config/lore/palettes/`, read at start.
- `c` on a page copies one of its examples to the clipboard: a list of them
  opens, Enter copies, and the key bar says what. The terminal does the copy
  (OSC 52), so it works over ssh and inside tmux, with no clipboard tool.
- The name drawn: "lore" in the 8-bit Press Start 2P letters, from the
  font's own pixels, beside the search bar on every screen, always in
  lore's orange (#e8793a) whatever the palette, and an about screen (`+`).
  The key bar shows only the keys. The about screen shows the
  version, what lore found here, where it keeps things, licence and source.
- A settings file, `~/.config/lore/config` (`$XDG_CONFIG_HOME/lore/config`),
  where lore saves the palette for the next runs. `LORE_PALETTE=NAME` uses
  another palette for one run.

### Changed

- A release's notes are the Unreleased section of the CHANGELOG when the
  version has no section of its own yet: a release is just a tag.

- The ci, docs and scripts workflows run real checks on pushes to main and
  pull requests: format, clippy, tests, the constants rule and a run of the
  release binary; the docs' own links and the manual page; shellcheck and
  the content scripts.
- The minimum Rust is 1.88, not 1.85: ratatui 0.30.2 needs it.

- `lore NAME` (and `lore git commit`) opens the page with the whole app
  around it: ← or Esc go back to the list instead of quitting, and every
  key works as usual; Esc in the list quits.

- The built-in pages are packed with zstd at build time and unpacked once
  when lore starts (about 50 ms), and the release build optimises the whole
  program as one unit (thin LTO, one codegen unit): the binary goes from
  27.0 MB to 8.2 MB.

### Fixed

- `lore check` no longer takes a redirection from a file named like a tag
  (`scheme <script.scm`) for HTML.
- The bash completion takes each candidate as it is: `[`, a command, was
  read as a file pattern.

## [0.1.0] - 2026-11-23

### Added

- **Palettes for colour blindness and low vision:** `modus-vivendi`,
  `modus-operandi` and their `-deuteranopia` and `-tritanopia` versions
  (every colour at least 7:1, WCAG AAA), `github-dark-colorblind`,
  `github-light-colorblind`, `github-dark-high-contrast`,
  `github-light-high-contrast`, and `okabe-ito` (the colour set made to
  stay apart in every kind of colour blindness). 33 palettes in all.
- **8 more palettes:** `monokai`, `github-dark`, `github-light`,
  `everforest-dark`, `everforest-light`, `kanagawa`, `ayu-dark` and
  `ayu-light` (23 in all).
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
