# git show

Show various types of Git objects (commits, tags, etc.).

## Usage
- `git show` : show information about the latest commit (hash, message, changes, and other metadata)
- `git show commit|tag|branch` : show information about a specific commit, tag, or branch (such as `HEAD` for the latest commit)
- `git show --summary commit` : show only the list of added, renamed, or deleted files
- `git show -w` : ignore whitespace when comparing lines
- `git show --oneline -s commit` : show a commit's message in a single line, suppressing the diff output
- `git show --stat commit` : show diff statistics for changed files (such as lines added and removed)
- `git show --name-only commit` : show a simplified list of all files changed in a commit (modified, added, and deleted)
- `git show revision:path/to/file` : show the contents of a file as it was at a given revision (e.g. branch, tag, or commit)

## Categories
version-control, files, text

## Links
- More information: https://git-scm.com/docs/git-show
- tldr page: https://tldr.inbrowser.app/pages/common/git-show
- Wikipedia: https://en.wikipedia.org/wiki/Git
