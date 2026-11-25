---
name: lore
description: Look up command-line tools offline with the `lore` command instead of the web. Use when the user asks what a command does, which tool does a task ("how do I compress a folder", "what converts images"), how to use a command or its flags, what a command line means, whether a tool is installed or up to date, or for a shell recipe or a cheat sheet (docker, git, ssh keys, postgresql). Prefer it over a web search for any question about a CLI tool on this machine.
---

# lore: offline answers about command-line tools

`lore` knows every command on this machine: what it does, its package, the
installed version and how to update it, common examples, related tools and
the man page. It also has recipes (tasks done in steps) and cheat sheets for
everyday tools. Answer CLI questions from it, not from the web.

## Before anything
The plugin gets lore ready by itself: when a session starts it installs lore
(the latest release, for this user, into `~/.local/bin`) if it is missing,
and says "lore is ready: PATH". Run lore by that path; below it is `$L`:

```bash
L=/the/path/it/said/lore   # else: L=$(command -v lore || echo ~/.local/bin/lore)
```

If the session start said lore could not be installed, run `/lore:setup`
(or ask the user to). If `"$L" --help` does not list `lore search`, it is an
older release: use `-p` and `explain`, and for "which tool" guess two or
three names and read their pages.

Never run bare `lore` or `lore NAME`: they open a full-screen program and
wait for keys. Use only the forms below, which print plain text and exit.

## Commands
| Need | Run |
|---|---|
| Which tool does X? | `"$L" search compress folder` |
| One tool's page (what, version, examples, related) | `"$L" -p NAME` |
| A subcommand's page | `"$L" -p git commit` |
| A recipe or a cheat sheet found by the search | `"$L" -p "compress a folder"`, `"$L" -p "docker (tool)"` |
| What each word and flag of a line means | `"$L" explain "tar -xzvf a.tgz"` |

`lore search WORDS` prints the best matches first, at most 25, one a line:
the name, the page line that matched (or its summary), and its categories,
separated by tabs. Names can have spaces (`git commit`, `docker compose`);
pass them to `lore -p` whole. It exits 1 when nothing matches: try other
words (lore knows that folder means directory, delete means remove).

## How to answer
1. For "which tool", run `lore search`, then `lore -p` on the two or three
   best names.
2. Read the VERSION section: suggest the tool that is installed. "Not
   installed on this machine" means it is not; give its install line only
   if the user wants it.
3. Quote the examples from the page instead of making flags up. When the
   page lacks something, read `man NAME` or `NAME --help` next, and go to the
   web only when the user asks.
4. Examples marked `!` are risky (they delete or overwrite): say so when
   you suggest one. lore never runs a command; neither should you, unless
   the user asks.
