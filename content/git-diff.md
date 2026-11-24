# git diff

Show changes to tracked files.

## Usage
- `git diff` : show unstaged changes
- `git diff HEAD` : show all uncommitted changes (including staged ones)
- `git diff --staged` : show only staged (added, but not yet committed) changes
- `git diff 'HEAD@{3 months|weeks|days|hours|seconds ago}'` : show changes from all commits since a given date/time (a date expression, e.g. "1 week 2 days" or an ISO date)
- `git diff --stat commit` : show diff statistics, like files changed, histogram, and total line insertions/deletions
- `git diff --summary commit` : output a summary of file creations, renames, and mode changes since a given commit
- `git diff branch_1..branch_2 path/to/file` : compare a single file between two branches or commits
- `git diff other_branch:path/to/file2 path/to/file1` : compare different files from the current branch to another branch

## Categories
version-control, files

## Links
- More information: https://git-scm.com/docs/git-diff
- tldr page: https://tldr.inbrowser.app/pages/common/git-diff
- Wikipedia: https://en.wikipedia.org/wiki/Git
