# git switch

Switch between Git branches. Requires Git version 2.23+.

## Usage
- `git switch branch_name` : switch to an existing branch
- `git switch -c branch_name` : create a new branch and switch to it
- `git switch -c branch_name commit` : create a new branch based on an existing commit and switch to it
- `git switch -` : switch to the previous branch
- `git switch --recurse-submodules branch_name` : switch to a branch and update all submodules to match
- `git switch -m branch_name` : switch to a branch and automatically merge the current branch and any uncommitted changes into it
- `git switch -d tag|commit` : switch to a tag or commit
- `git switch -t remote_name/branch` : switch to another remote's branch

## See also
- git checkout

## Categories
version-control

## Links
- More information: https://git-scm.com/docs/git-switch
- tldr page: https://tldr.inbrowser.app/pages/common/git-switch
