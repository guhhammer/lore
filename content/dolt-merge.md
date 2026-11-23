# dolt merge

Join two or more development histories together.

## Usage
- `dolt merge branch_name` : incorporate changes from the named commits into the current branch
- `dolt merge --squash branch_name` : incorporate changes from the named commits into the current branch without updating the commit history
- `dolt merge --no-ff branch_name` : merge a branch and create a merge commit even when the merge resolves as a fast-forward
- `dolt merge --no-ff -m "message" branch_name` : merge a branch and create a merge commit with a specific commit message
- `dolt merge --abort` : abort the current conflict resolution process

## Categories
version-control

## Links
- More information: https://docs.dolthub.com/cli-reference/cli#dolt-merge
- tldr page: https://tldr.inbrowser.app/pages/common/dolt-merge
