# git merge

Merge branches.

## Usage
- `git merge branch_name1 branch_name2 ...` : merge branches into the current branch
- `git merge -e branch_name` : edit the merge message
- `git merge --no-ff branch_name` : merge a branch and create a merge commit
- `git merge --squash branch_name` : stage the result of merging a branch without creating a commit
- `git merge --abort` : abort a merge in case of conflicts
- `git merge -s strategy -X strategy_option branch_name` : merge using a specific strategy

## Categories
version-control

## Links
- More information: https://git-scm.com/docs/git-merge
- tldr page: https://tldr.inbrowser.app/pages/common/git-merge
