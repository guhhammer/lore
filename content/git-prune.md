# git prune

Git command for pruning all unreachable objects from the object database. This command is often not used directly but as an internal command that is used by Git gc.

## Usage
- `git prune -n` : simulate pruning, reporting what would be removed
- `git prune -v` : prune unreachable objects and display what has been pruned to `stdout`
- `git prune --progress` : prune unreachable objects while showing progress

## Categories
version-control, databases

## Links
- More information: https://git-scm.com/docs/git-prune
- tldr page: https://tldr.inbrowser.app/pages/common/git-prune
