# git name-rev

Describe a commit using existing ref names.

## Usage
- `git name-rev HEAD` : show the name for `HEAD`
- `git name-rev --name-only HEAD` : show only the name
- `git name-rev --all` : enumerate all matching ref names
- `git name-rev --tags HEAD` : use only tags to name the commit
- `git name-rev --no-undefined commit-ish` : exit with a non-zero status code instead of printing `undefined` for unknown commits
- `git name-rev HEAD~1 HEAD~2 main` : show names for multiple commits
- `git name-rev --refs refs/heads/ commit-ish` : restrict names to branch refs
- `echo "commit-ish" | git name-rev --annotate-stdin` : read commit IDs from `stdin`

## Categories
version-control

## Links
- More information: https://git-scm.com/docs/git-name-rev
- tldr page: https://tldr.inbrowser.app/pages/common/git-name-rev
