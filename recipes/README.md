# Recipes

Short task pages that chain commands: "free disk space", "what is using a
port", "why won't a service start". Every `*.md` here is built into lore
(`build.rs` checks it), and shows in the list under the `recipes` header,
found by the search like any command and opened the same way.

## The format

The same as a command's page in `content/` (see
[`content/README.md`](../content/README.md)), with **Steps** for Usage:

```markdown
# free disk space

One or two sentences: what the recipe does, in order.

## Steps
- `df -h` : what this step shows or does
- `sudo journalctl --vacuum-size=200M` : ...

## Notes
- Distribution differences, macOS, what to put in place of NAME or PID.

## See also
- df
- du

## Categories
disks files system
```

- **The title** (`# …`) is the recipe's name in the list: lower case, a
  task as people say it. The file name is the title with dashes.
- **Steps** run in order, each one command a person can type, with what it
  shows or does. Write `NAME`, `PID` or a real example in place of what
  changes, and say so in the notes. Risky steps (`rm -rf`, `kill -9`) are
  marked on the page by lore itself.
- **Linux first**, the distribution named when a step is for one; macOS in
  the notes.
- Every step is checked by hand before a recipe is sent.
