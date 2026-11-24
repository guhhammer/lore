# Security policy

## Reporting a vulnerability

**Please do not open a public issue.** Report it privately instead:
GitHub → the **Security** tab → **Report a vulnerability**.

Say what you did, what happened and what you expected; a proof of concept
helps. You'll get a reply within 7 days. Please allow a reasonable time for
a fix before telling others.

**Supported versions:** only the latest release and `main`. There are no
release branches, so fixes land on `main` and in the next release.

---

## What lore is

A terminal program that each person runs on their own machine, as
themselves. Nothing else can reach it: **it listens on no port**. A
vulnerability is anything that makes it do more than what this page
describes.

### What it runs

lore **never runs, opens or changes the commands and files it lists.** It
runs only these, each with a plain argument and never through a shell:

- `man` and `whatis`, to read a command's manual;
- read-only package queries: `dpkg-query`, `apt-cache policy`, `rpm -qf`
  and `pacman -Qo`. Homebrew is read from its folders, without running it.

### What it reads

- `$PATH`, `$HOME`, `$SHELL`, `$LANG` and the XDG folder variables;
- the names of the files in the program folders (`/usr/bin`, `/usr/sbin`,
  Homebrew, Nix…), and on macOS `/etc/paths` and `/etc/paths.d/*`;
- your shell's startup files, keeping only their `alias` lines;
- your shell history (bash, zsh, fish, `$HISTFILE`), to count which
  commands you use most. It is only counted, never kept or sent. History
  off on the `+` screen stops this;
- pages from `~/.local/share/lore/content/` and
  `/usr/share/lore/content/`;
- its own settings and lists (below).

### What it writes

Only its own files, and only when you change something:

| File | When |
| --- | --- |
| `~/.config/lore/config` | You change a setting on the `+` screen. |
| `~/.local/share/lore/bookmarks`, `recent` | You bookmark or open a page. |
| `~/.cache/lore/internet/NAME.txt` | A cheat sheet arrives from the internet (kept a day). |

It reads no passwords, keys or tokens.

### What it sends

Nothing, unless the internet is on (it is by default):

- a connection test to `cht.sh` on port 443 every 30 seconds, for the
  green or red dot;
- when you open a command's page, one https request to
  `https://cht.sh/NAME?T`, made by lore itself (rustls), never by running a
  program. It carries the command's name, and lore's name and version as
  its user agent; nothing about you.

Turn **Internet off** on the `+` screen (`internet = off` in the settings
file), and lore makes no network request at all.

### The clipboard

`c` on a page copies one of **that page's own examples**, by asking the
terminal (the OSC 52 escape). Nothing else is ever copied.

---

## What is in scope

- **Text that drives your terminal.** Everything lore draws is meant to be
  plain text: command names, aliases, pages, `man` output and the internet
  text all have control, escape and invisible (bidirectional, zero-width)
  characters removed. Any of them reaching the terminal as something other
  than text is in scope.
- **The INTERNET section.** cheat.sh is written by its community and its
  answer is not checked. lore shows it as raw lines behind a bar, never
  turns it into links, never copies it and never runs it, and the section
  says so. Text from it that does more, or any request made with the
  internet off, is in scope.
- **Pages from other programs** (`/usr/share/lore/content/`). Each must
  pass `lore check`, and may only fill in a command lore has no page for,
  never replace one. A page that gets past the check with escape codes,
  hidden text, HTML, script, an example that runs a download
  (`curl … | sh`), or that replaces another command's page, is in scope.
- **Links.** lore shows only `https://` links and never opens one; your
  terminal does, when you Ctrl+click. A page that leads to a log-in or
  collects anything from the reader is in scope: lore never asks anyone to
  log in.
- **Anything lore runs, opens or writes** beyond the lists above.

## Out of scope

- Bugs in ratatui, crossterm, rustls or the Rust standard library: please
  report those to their projects.
- Attacks that need the attacker to already control your account or
  machine.
- Your own folder `~/.local/share/lore/content/` replacing built-in pages:
  that is a feature, and only you can write there.
