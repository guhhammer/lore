# makoctl mode

Manage notification modes in `mako`. Modes can be used to change notification behavior (e.g., do-not-disturb).

## Usage
- `makoctl mode` : list all currently active modes
- `makoctl mode -a do-not-disturb` : add a mode
- `makoctl mode -r do-not-disturb` : remove a mode
- `makoctl mode -t do-not-disturb` : toggle a mode (add if absent, remove if present)
- `makoctl mode -s mode1 mode2 ...` : set specific modes, replacing all current modes

## Categories
desktop

## Links
- More information: https://manned.org/makoctl
- tldr page: https://tldr.inbrowser.app/pages/linux/makoctl-mode
