# Recipes

Short task pages that chain commands, written the way people ask:
*free disk space*, *what is using a port*, *why won't a service start*.
There are 100, most of them the terminal tasks with the most-viewed Stack
Overflow questions.

In lore they sit in the list under **recipes**, the search finds them like
any command, and each opens as a page with its **STEPS**, the commands it
**USES** (each one a page of its own) and its **NOTES**.

Every file here is built into lore, and `build.rs` checks it: a broken
recipe stops the build with the file named.

---

## Add a recipe

One file, named with dashes: `free-disk-space.md`.

```markdown
# free disk space

One or two sentences: what the recipe does, in order.

## Steps
- `df -h` : see which disk is full
- `sudo du -xh / | sort -h | tail -20` : find the biggest folders on it
- `sudo journalctl --vacuum-size=200M` : shrink the system logs

## Notes
- Distribution differences, macOS, what to put in place of NAME or PID.

## See also
- df
- du

## Categories
disks files system
```

| Part | Rule |
| --- | --- |
| `# title` | The name shown in the list: lower case, a task as people say it. |
| `## Steps` | At least one. Each is one command a person can type, then what it shows or does, in order. |
| `## Notes` | Where to put your own value (`NAME`, `PID`), and differences between distributions and macOS. |
| `## See also`, `## Categories` | As in a command's page ([content/README.md](../content/README.md)). |

**Guidelines**

- **Linux first.** Name the distribution when a step is for one; put macOS
  in the notes.
- **Write `NAME`, `PID`** or a real example in place of what changes, and
  say so in the notes.
- **Risky steps** (`rm -rf`, `kill -9`) are marked with `!` by lore itself;
  still say in the notes what they do.
- **Try every step** before you send the recipe.
