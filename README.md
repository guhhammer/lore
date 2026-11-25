# lore for Claude Code

**Offline answers about command-line tools, from the `lore` command: which
tool does a task, what a command or a flag does, whether it is installed.**

![lore in 20 seconds: search "compress folder" and tar comes first, its page, a tool's cheat sheet, the languages, and the agent plugin](.github/media/lore-2.svg)

[lore](.github/README.md) is a terminal app that lists every command on your
machine, A to Z, with a short, practical page for each: what it does, the
examples you need, its package and version, and what can go wrong. This
repository is also a Claude Code plugin: its skill answers command-line
questions from lore instead of the web.

## Install

```
/plugin marketplace add guhhammer/lore
/plugin install lore@lore
```

That is all. When a session starts, the plugin checks for lore and, when it
is missing or too old, installs the latest release for your system:

- for you only, with no sudo: the binary into `~/.local/bin`, the manual
  into `~/.local/share/man/man1`;
- checked against the release's `SHA256SUMS` (nothing is installed when it
  does not match);
- once: the next sessions find it and download nothing.

A lore you installed from a package (.deb, .rpm, Homebrew, see
[Install](.github/README.md#install)) is used as it is.

## What it adds

| Part | What it does |
| --- | --- |
| Skill `lore` | Used when you ask about a command-line tool: "how do I compress a folder", "what does `tar -xzvf` mean", "is ripgrep installed". |
| Command `/lore:setup` | Installs lore by hand; `/lore:setup --update` gets the latest release. |
| Session hook | Gets lore ready when a session starts and tells the skill where it is. |

The skill runs only three commands, all of which print plain text and exit:

| Command | Gives |
| --- | --- |
| `lore search compress folder` | The best matching tools, recipes and cheat sheets, one a line. |
| `lore -p tar` | One tool's page: what it does, the installed version and how to update it, examples, related tools. |
| `lore explain "tar -xzvf a.tgz"` | What each word and flag of a line means, from the man page. |

lore never runs the commands it lists, and the skill does not either unless
you ask. Examples that delete or overwrite are marked as risky, and the skill
says so when it suggests one.

## Privacy

Everything the skill reads is on your machine: lore's pages are built into
the binary. The only network use is the download of a release from
`github.com/guhhammer/lore` when lore is missing. More in
[SECURITY.md](.github/SECURITY.md).

## Other tools

The skill is one Markdown file,
[`claude-plugin/skills/lore/SKILL.md`](claude-plugin/skills/lore/SKILL.md);
other tools that read skills can use it as it is.

## More

- [lore's README](.github/README.md): the app, its keys, settings and pages.
- [CHANGELOG.md](CHANGELOG.md): what changed in each version.
- Licence: MIT, see [LICENSE](LICENSE).
