# git

git tracks the history of a project. This page holds the moves people look up most: saving work, undoing mistakes, branching, rewriting history and working with remotes.

## Tasks
### Save work
- `git status` : what changed and what is staged
- `git add -p` : choose which pieces of a file to stage
- `git commit -m "message"` : save the staged changes
- `git diff` : changes not yet staged
- `git log --oneline --graph --all` : the history as a short graph
### Undo
- `git restore FILE` : throw away the changes in a file (deletes them)
- `git restore --staged FILE` : unstage a file, keep the changes
- `git reset --soft HEAD~1` : undo the last commit, keep the changes staged
- `git reset --hard HEAD~1` : undo the last commit and its changes (deletes them)
- `git revert COMMIT` : make a new commit that cancels an old one
- `git reflog` : every place HEAD has been, to recover lost commits
### Amend and rebase
- `git commit --amend` : change the last commit's message or content
- `git commit --amend --no-edit` : add staged changes to the last commit
- `git rebase -i HEAD~3` : reorder, squash or reword the last three commits
- `git rebase main` : replay your branch on top of main
### Stash
- `git stash push -u -m "NAME"` : set the changes aside, untracked files too
- `git stash pop` : bring the latest stash back and drop it
### Branches
- `git switch -c NAME` : make a branch and move to it
- `git merge NAME` : merge a branch into the current one
- `git branch -D NAME` : delete a branch even if unmerged (deletes its work)
### Remotes
- `git clone URL` : copy a repository
- `git pull --rebase` : fetch and replay your commits on top
- `git push -u origin NAME` : publish a branch and track it
- `git push --force-with-lease` : force a push only if nobody else pushed
### Find a bug
- `git bisect start` : begin a binary search through history
- `git bisect bad` : mark the current commit as broken
- `git bisect good COMMIT` : mark a commit known to work
- `git bisect reset` : end the search

## Notes
- `reset --hard`, `clean -fd` and `branch -D` throw work away: check `git status` first.
- Never rewrite commits that others already pulled; use `revert` for those.
- A lost commit is usually still in `git reflog` for a few weeks.
- `git push --force` can overwrite other people's work; `--force-with-lease` is the safer form.

## See also
- gh
- tig
- diff

## Links
- Reference: https://git-scm.com/docs
- Pro Git book: https://git-scm.com/book

## Categories
development
