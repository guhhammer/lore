# lore's pages

Every command page lore shows is a small Markdown file here: `ls.md` is the
page for `ls`. They are built into lore when it is compiled, so lore needs
no folder to show them.

- [Write or fix a page](#the-format)
- [A page for your own program](#a-page-for-your-own-program)
- [Where the pages came from](#where-the-pages-came-from)

---

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

| Section | How to write it |
| --- | --- |
| `# name` | The command, exactly as typed. A subcommand has a space: `# git commit`, in the file `git-commit.md`. |
| Summary | A few plain sentences under the title. |
| `## Usage` | One example a line: `` - `what to type` : what it does ``. Most common first. |
| `## Notes` | What can go wrong, what cannot be undone, differences between systems. |
| `## See also` | Related commands, one a line. lore adds the man page's SEE ALSO. |
| `## Categories` | One line, from the list below. |
| `## Links` | One a line: `- label: https://…`. The program's own site and docs. lore adds Stack Overflow, GeeksforGeeks and Google searches by itself. |

Any other `##` section is ignored. The version, the package and the manual
are not written here: lore reads them from the machine.

**Categories:** files, text, archives, network, processes, system, disks,
users, packages, development, version-control, containers, cloud, security,
media, shell, editors, monitoring, hardware, databases, desktop, documents.
Add `danger` for a command that can destroy data or stop the system: it is
shown in red and the page warns. Add macos, windows, android, bsd or sunos
for a command of that system only. Most categories were set automatically,
so corrections are welcome.

**Good to know**

- **Links** must be `https://`; others are not shown. Keep them short: a
  link wider than the screen gets cut.
- **Translations** go in `i18n/<language>/<name>.md` (`i18n/pt_BR/tar.md`)
  with the summary, usage and links in that language. lore picks the
  language from `LC_ALL`, `LC_MESSAGES` or `LANG`; the rest comes from the
  English page.
- **Pages for other systems** (Windows, BSD, SunOS, Android) say so in
  their Notes. They open with `lore NAME` but are not listed, since the
  command doesn't run here.

**Before you send a page**

- Every link goes where it says, over https, with no tracking.
- Nothing asks the reader to log in, sign up or pay.
- Examples are safe to try, or say plainly when they are not. lore marks
  risky ones (`rm -rf`, `dd`…) with `!` by itself.

---

## A page for your own program

Ship a page with your program and lore shows it for your command.

| Where | Used how |
| --- | --- |
| `/usr/share/lore/content/NAME.md` (or `/usr/local/share/…` for a local install) | Shown only when lore has no page of its own for `NAME`: a package can add a page, never replace one. |
| `/usr/share/lore/content/i18n/<lang>/NAME.md` | Its translations. |
| `~/.local/share/lore/content/NAME.md` | The reader's own pages, which can replace any page, built-in ones too. |

**Run `lore check NAME.md` before you ship it.** It exits with 1 on an
error, so you can run it in your build. A page that fails is not shown,
because a page must be plain text that a terminal shows and does not obey.

| Error: the page is not shown | Why |
| --- | --- |
| A title other than `# NAME` | A page may not pose as another command's. |
| Control characters, escape sequences | A terminal obeys them: it could clear the screen, set the clipboard, change the title. |
| Invisible characters (zero-width, bidirectional) | They hide text or reorder what is shown ("Trojan Source"). |
| HTML or script (`<script>`, `javascript:`…) | lore shows text; nothing in a page is ever run. |
| An example that runs a download (`curl … \| sh`, `bash <(wget …)`) | No page may hand the reader a command that runs a server's code. |
| Over 64 KB, or not UTF-8 | A page is a short text. |

| Warning: shown, but check it | |
| --- | --- |
| A risky example (`rm -rf`, `dd`…) | lore marks it with `!`. |
| A link that isn't https | lore doesn't show it. |
| A user or password in a link, a punycode (`xn--`) or bare-address host | It can hide where the link really goes. |
| A link over 76 characters | A narrow screen cuts it. |

---

## Where the pages came from

- **Written for lore:** `cd`, `chmod`, `cp`, `find`, `grep`, `lore`, `ls`,
  `mkdir`, `mv`, `rm`, `ssh`, `sudo`, `systemctl`, `tar`.
- **Most examples** are adapted from
  [tldr-pages](https://github.com/tldr-pages/tldr) (commit c90cc0b), under
  [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/). What changed:
  converted to lore's format, placeholders such as `{{path/to/file}}`
  became plain words, pointer pages were filled from the page they point
  to, and long pages were cut to 8 examples. tldr's Windows, BSD, SunOS and
  Android pages are included for names no other page uses.
- **The translations** in `i18n/` are tldr-pages' own, in 41 languages.
- **Commands tldr doesn't cover** got pages written from their local manual
  pages.

The scripts that made them are in [`scripts/content/`](../scripts/content/).
