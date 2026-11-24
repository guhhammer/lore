# git branch

Main Git command for working with branches.

## Usage
- `git branch -a` : list all branches (local and remote; the current branch is highlighted by `*`)
- `git branch -a --contains commit_hash` : list which branches include a specific Git commit in their history
- `git branch --show-current` : show the name of the current branch
- `git branch branch_name` : create new branch based on the current commit
- `git branch branch_name commit_hash` : create new branch based on a specific commit
- `git branch -m old_branch_name new_branch_name` : rename a branch
- `git branch -d branch_name` : delete a local branch (you must switch to a different branch before doing this)
- `git push remote_name -d remote_branch_name` : delete a remote branch

## Categories
version-control

## Links
- More information: https://git-scm.com/docs/git-branch
- tldr page: https://tldr.inbrowser.app/pages/common/git-branch
- Wikipedia: https://en.wikipedia.org/wiki/Git
