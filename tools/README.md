# Tools

Cheat sheets of the everyday tasks a tool is used for: the things people
look up again and again (*make an ssh key*, *back up a PostgreSQL
database*, *see what is listening on a port*). One page per tool.

In lore they are the **tools** topic (← → in the list), the search finds
them like any command, and each opens as a page with its **TASKS** in
groups (Tab jumps from group to group), its **NOTES**, the commands it is
**RELATED** to and its **LINKS**. `c` copies a task's command.

Every file here is built into lore, and `build.rs` checks it: a broken
page stops the build with the file named.

---

## Add a tool page

One file, named with dashes: `ssh-keys.md`.

```markdown
# ssh keys

One or two sentences: what the tool is and what this page covers.

## Tasks
### Make a key
- `ssh-keygen -t ed25519 -C "you@laptop"` : make a key pair; Enter keeps the default file
### Use it
- `ssh-copy-id user@host` : put your public key on a server

## Notes
- One fact a newcomer trips over, per line.

## See also
- ssh
- ssh-keygen

## Links
- Manual: https://man.openbsd.org/ssh-keygen

## Categories
network security
```

- `## Tasks` holds the tasks, `- `command` : what it does`, under `### `
  group titles. Write placeholders in capitals (`USER`, `HOST`, `FILE`).
- A name already used by a command (`curl`) is listed as `curl (tool)`.
- Links are `https://` only. Nothing that downloads and runs a script
  (`curl … | sh`): lore's check refuses it.
- Prefer the commands every system has; say when one is for one system
  only (Debian, Fedora, macOS).
