# git cherry-pick

Apply the changes introduced by existing commits to the current branch. To apply changes to another branch, first use `git checkout` to switch to the desired branch.

## Usage
- `git cherry-pick commit` : apply a commit to the current branch
- `git cherry-pick start_commit~..end_commit` : apply a range of commits to the current branch (see also: `git rebase --onto`)
- `git cherry-pick commit1 commit2 ...` : apply multiple (non-sequential) commits to the current branch
- `git cherry-pick -n commit` : add the changes of a commit to the working directory, without creating a commit
- `git cherry-pick -x commit` : append a line to the commit message to indicate that the commit was cherry-picked

## Categories
version-control

## Links
- More information: https://git-scm.com/docs/git-cherry-pick
- tldr page: https://tldr.inbrowser.app/pages/common/git-cherry-pick
