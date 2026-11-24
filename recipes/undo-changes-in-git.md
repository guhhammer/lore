# undo changes in git

Throw away changes not committed yet, unstage files, or put work aside for later.

## Steps
- `git status` : what changed, staged or not
- `git restore file.txt` : throw away one file's changes
- `git restore --staged file.txt` : unstage it, keeping the changes
- `git stash` : put every change aside; git stash pop brings them back
- `git clean -nd` : list the new files git does not track
- `git clean -fd` : delete them
- `git reset --hard` : throw away every change to tracked files

## Notes
- restore, clean -fd and reset --hard cannot be undone for changes never committed.

## See also
- git

## Categories
version-control
