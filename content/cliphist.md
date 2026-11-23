# cliphist

Manage clipboard history for Wayland compositors. Works with `wl-copy` and `wl-paste`.

## Usage
- `cliphist list` : list clipboard history entries
- `cliphist list | fzf | cliphist decode | wl-copy` : select and copy a previous clipboard entry (using `fzf`)
- `cliphist wipe` : delete all stored clipboard entries
- `echo entry_id | cliphist delete` : delete a specific clipboard entry by ID
- `wl-paste | cliphist store` : save the current clipboard content manually

## Categories
desktop, files, shell

## Links
- More information: https://github.com/sentriz/cliphist#usage
- tldr page: https://tldr.inbrowser.app/pages/linux/cliphist
