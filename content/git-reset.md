# git reset

Undo commits or unstage changes by resetting the current Git HEAD to the specified state. If a path is passed, it works as "unstage"; if a commit hash or branch is passed, it works as "uncommit".

## Usage
- `git reset` : unstage everything
- `git reset path/to/file1 path/to/file2 ...` : unstage specific file(s)
- `git reset -p path/to/file` : interactively unstage portions of a file
- `git reset HEAD~` : undo the last commit, keeping its changes (and any further uncommitted changes) in the filesystem
- `git reset --soft HEAD~2` : undo the last two commits, adding their changes to the index, i.e. staged for commit
- `git reset --hard` : discard any uncommitted changes, staged or not (for only unstaged changes, use `git checkout`)
- `git reset --hard commit` : reset the repository to a given commit, discarding committed, staged, and uncommitted changes since then

## Categories
version-control, files, security

## Links
- More information: https://git-scm.com/docs/git-reset
- tldr page: https://tldr.inbrowser.app/pages/common/git-reset
- Wikipedia: https://en.wikipedia.org/wiki/Git
