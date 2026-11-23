# git rebase

Reapply commits from one branch on top of another branch. Commonly used to "move" an entire branch to another base, creating copies of the commits in the new location.

## Usage
- `git rebase new_base_branch` : rebase the current branch on top of another specified branch
- `git rebase -i target_base_branch_or_commit_hash` : start an interactive rebase, which allows the commits to be reordered, omitted, combined, or modified
- `git rebase --continue` : continue a rebase that was interrupted by a merge failure, after editing conflicting files
- `git rebase --skip` : continue a rebase that was paused due to merge conflicts, by skipping the conflicted commit
- `git rebase --abort` : abort a rebase in progress (e.g. if it is interrupted by a merge conflict)
- `git rebase --onto new_base old_base` : move part of the current branch onto a new base, providing the old base to start from
- `git rebase -i HEAD~5` : reapply the last 5 commits in-place, stopping to allow them to be reordered, omitted, combined, or modified
- `git rebase -X theirs branch_name` : auto-resolve any conflicts by favoring the working branch version (`theirs` keyword has reversed meaning in this case)

## Categories
version-control, files

## Links
- More information: https://git-scm.com/docs/git-rebase
- tldr page: https://tldr.inbrowser.app/pages/common/git-rebase
