# dolt branch

Manage Dolt branches.

## Usage
- `dolt branch` : list local branches (current branch is highlighted by `*`)
- `dolt branch -A` : list all local and remote branches
- `dolt branch branch_name` : create a new branch based on the current branch
- `dolt branch branch_name commit` : create a new branch with the specified commit as the latest
- `dolt branch -m branch_name1 branch_name2` : rename a branch
- `dolt branch -c branch_name1 branch_name2` : duplicate a branch
- `dolt branch -d branch_name` : delete a branch
- `dolt branch --show-current` : display the name of the current branch

## Categories
version-control

## Links
- More information: https://docs.dolthub.com/cli-reference/cli#dolt-branch
- tldr page: https://tldr.inbrowser.app/pages/common/dolt-branch
