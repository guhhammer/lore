# git status

Show the changes to files in a Git repository. List changed, added, and deleted files compared to the currently checked-out commit.

## Usage
- `git status` : show untracked and changed files which are not yet added for commit
- `git status -s` : give output in short format
- `git status -sb` : show output in short format along with branch info
- `git status -v` : show the changes in staged files (like `git diff --cached`)
- `git status -vv` : show the changes in all tracked files (like `git diff HEAD`)
- `git status --show-stash` : show the number of entries currently stashed away
- `git status -uno` : don't show untracked files in the output

## Categories
version-control, packages

## Links
- More information: https://git-scm.com/docs/git-status
- tldr page: https://tldr.inbrowser.app/pages/common/git-status
- Wikipedia: https://en.wikipedia.org/wiki/Git
