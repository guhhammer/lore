---
description: Install or update lore, the offline command-line reference the lore skill reads
allowed-tools: Bash(bash:*)
---

Run the plugin's setup script and report what it says, in one or two lines:

```bash
bash "${CLAUDE_PLUGIN_ROOT}/scripts/setup.sh" $ARGUMENTS
```

- With no argument it installs lore when it is missing or too old (the
  latest release, for this user only, into `~/.local/bin`, checked against
  the release's SHA256SUMS, no sudo), and otherwise says where it is.
- `/lore:setup --update` installs the latest release even when lore is there.

If it says the folder is not on PATH, tell the user the line to add to their
shell's startup file (`export PATH="$HOME/.local/bin:$PATH"`), but do not
edit the file unasked. If it could not download (offline, or no release for
this system), say so and point to https://github.com/guhhammer/lore#install.
