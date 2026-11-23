# Security Policy

## Reporting a vulnerability

**Do not open a public issue.** Report privately: GitHub → the **Security**
tab → *Report a vulnerability* (a private advisory).

Include what you did, what happened, and what you expected. A proof of
concept helps. You'll get an acknowledgement within 7 days.

Please give a fix a reasonable window before disclosing publicly.

## Supported versions

Only `main`. Versions are recorded in `CHANGELOG.md`, not as release
branches, so there are no backports — fixes land on the tip.

## Scope

`lore` is a terminal program each user runs on their own machine, as
themselves. It reaches no network and nobody else reaches it, so a
vulnerability here is something that makes it do more than read directory
listings and draw them.

**In scope:**

- The names of files in the program directories, and the aliases in the
  shell's startup files: they are drawn to the terminal as they are, so a
  name or an alias holding terminal escape sequences is in scope.
- `lore` must never run, open or write any of the files it lists.

**Out of scope:**

- Vulnerabilities in ratatui, crossterm or the Rust standard library —
  report those upstream.
- Anything requiring the attacker to already control the user's account or
  the host.

## What listens

Nothing in it listens on a port.

## Handling secrets

`lore` reads no credentials and writes nothing. What it reads: `$PATH`,
`$HOME` and `$SHELL`; the listings of the program directories; `/etc/paths`
and `/etc/paths.d/*` (macOS); the shell's startup files (system-wide and the
user's), of which it keeps only the `alias` lines; and, for fish, the names
of the files in `~/.config/fish/functions/`. Every path is listed in
`src/compile_config.rs`. If a change ever makes it read a secret, this
section says where it lives and how it is kept out of the repository.
