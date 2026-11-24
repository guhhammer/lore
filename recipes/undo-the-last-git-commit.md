# undo the last git commit

Take back a commit, keep its changes or not, fix its message, or undo one already pushed.

## Steps
- `git reset --soft HEAD~1` : undo it, keeping its changes staged
- `git reset HEAD~1` : undo it, keeping the changes, unstaged
- `git commit --amend` : change its message, or add what you forgot (git add first)
- `git revert HEAD` : already pushed: a new commit that undoes it
- `git reflog` : every place HEAD was: find a commit you lost

## Notes
- Do not reset or amend commits others already pulled; revert them.

## See also
- git

## Categories
version-control
