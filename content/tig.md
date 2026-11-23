# tig

A configurable `ncurses`-based TUI for Git.

## Usage
- `tig` : show the sequence of commits starting from the current one in reverse chronological order
- `tig branch` : show the history of a specific branch
- `tig path1 path2 ...` : show the history of specific files or directories
- `tig base_ref..compared_ref` : show the difference between two references (such as branches or tags)
- `tig blame path/to/file` : browse `git blame` interactively (press `<,>` to jump to parent)
- `tig --all` : display commits from all branches and stashes
- `tig stash` : start in stash view, displaying all saved stashes
- `<h>` : display help in TUI

## See also
- gitui
- git-gui

## Categories
version-control, files

## Links
- More information: https://jonas.github.io/tig/doc/manual.html
- tldr page: https://tldr.inbrowser.app/pages/common/tig
