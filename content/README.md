# lore's pages

One file per command, named after it: `ls.md` is the page for `ls`. Every
page here is built into lore when it is compiled, so lore needs no folder
to show them. To add or change a page on your own machine without
rebuilding, put it in `~/.local/share/lore/content/` (or, for every user,
`/usr/local/share/lore/content/`); a page there wins over the built-in one.

## Where the pages came from

- Hand-written: `cd`, `chmod`, `cp`, `find`, `grep`, `lore`, `ls`, `mkdir`, `mv`, `rm`, `ssh`, `sudo`, `systemctl`, `tar`.
- Most usage examples are adapted from [tldr-pages](https://github.com/tldr-pages/tldr)
  (commit c90cc0b), licensed under
  [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/). Changes: the
  pages were converted to lore's format, placeholders such as
  `{{path/to/file}}` became plain words, pointer pages were filled from the
  page they point to, and long pages were cut to 8 examples.
- The translations in `i18n/` are tldr-pages' own (41 languages), in
  lore's format.
- tldr's Windows, FreeBSD, OpenBSD, NetBSD, SunOS and Android pages are
  included too, for names no other page already uses.
- Pages for commands tldr does not cover were written from each command's
  local manual page, and some were revised or written by hand.

Corrections are welcome: a page is one small Markdown file.

## The format

```markdown
# ls

What the command is and does, in a few plain sentences. A blank line
starts a new paragraph.

## Usage
- `ls` : the files in the current folder
- `ls -la` : every file, hidden ones too, with details

## Notes
Anything important: what can go wrong, what cannot be undone.

## See also
- dir
- tree

## Categories
files

## Links
- GNU manual: https://www.gnu.org/software/coreutils/ls
```

- **Usage:** one line each, `` - `what to type` : what it does ``. Put the
  most common uses first.
- **Links:** one line each, `- label: https://…`. Only `https://` links are
  shown. Keep a URL short: one wider than the reader's screen is cut, and
  the terminal would open the cut one. Link the program's own site and
  documentation; lore adds Stack Overflow, GeeksforGeeks and Google
  searches to every page by itself.
- **See also:** related commands, one per line (`- tree`). lore adds the
  man page's SEE ALSO to them.
- **Categories:** one line, comma-separated, from this set: files, text,
  archives, network, processes, system, disks, users, packages,
  development, version-control, containers, cloud, security, media,
  shell, editors, monitoring, hardware, databases, desktop, documents;
  `danger` for a command that can destroy data or stop the system (it is
  shown in red, and the page warns); and macos, windows, android, bsd or
  sunos for a command of that system only. Categories show after the name
  in the list, and the search finds commands by them. Most were set by a
  keyword classifier: corrections are welcome.
- Any other `##` section is ignored.
- **Translations:** `i18n/<language>/<name>.md` (`i18n/pt_BR/tar.md`) holds
  a page's summary, usage and links in another language. lore picks the
  language from `LC_ALL`, `LC_MESSAGES` or `LANG`; the rest of the page
  comes from the English one.
- **Subcommands:** `git-commit.md` holds `git commit`, and its first line
  is `# git commit`. A title with a space makes the page a subcommand: it
  is listed under its program when that program is installed, opens with
  `lore git commit`, and is named on its program's page.
- **Other systems:** a page for a Windows, BSD, SunOS or Android command
  says so in its Notes. Those pages open with `lore NAME` but are not
  listed, since the command does not run here.

The version, the package and the manual page are not written here: lore
reads them from the machine.

## Before you send a page

- Every link goes to the page it says, over https, with no tracking.
- Nothing asks the reader to log in, sign up or pay. lore never does, and
  every page says so under its links.
- Examples are safe to try, or say plainly when they are not (`rm`, `dd`).
