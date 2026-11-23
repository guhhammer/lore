# git gui

A GUI for Git to manage branches, commits, and remotes, and perform local merges.

## Usage
- `git gui` : launch the GUI
- `git gui blame path/to/file` : show a specific file with author name and commit hash on each line
- `git gui blame revision path/to/file` : open `git gui blame` in a specific revision
- `git gui blame --line=line path/to/file` : open `git gui blame` and scroll the view to center on a specific line
- `git gui citool` : open a window to make one commit and return to the shell when it is complete
- `git gui citool --amend` : open `git gui citool` in the "Amend Last Commit" mode
- `git gui citool --nocommit` : open `git gui citool` in a read-only mode
- `git gui browser maint` : show a browser for the tree of a specific branch, opening the blame tool when clicking on the files

## See also
- git cola
- gitk

## Categories
version-control, files, text

## Links
- More information: https://git-scm.com/docs/git-gui
- tldr page: https://tldr.inbrowser.app/pages/common/git-gui
