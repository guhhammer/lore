# git stash

Stash local Git changes in a temporary area.

## Usage
- `git stash` : stash the current uncommitted changes
- `git stash -u` : stash current changes, including new untracked files
- `git stash -p` : interactively select parts of changed files for stashing
- `git stash list` : list all stashes
- `git stash show -p` : show the changes as a patch between the stash and the commit back when stash entry was first created
- `git stash pop` : apply a stash and remove it from the stash list if applying doesn't cause conflicts
- `git stash drop` : delete the latest stash
- `git stash clear` : delete all stashes

## Categories
version-control

## Links
- More information: https://git-scm.com/docs/git-stash
- tldr page: https://tldr.inbrowser.app/pages/common/git-stash
