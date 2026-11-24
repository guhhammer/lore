# git for-each-ref

List and optionally format references (branches, tags) in a Git repository.

## Usage
- `git for-each-ref` : list all refs (branches and tags)
- `git for-each-ref refs/heads/` : list only branches
- `git for-each-ref refs/tags/` : list only tags
- `git for-each-ref --merged HEAD refs/heads/` : show branches merged into `HEAD`
- `git for-each-ref --format "%(refname:short)"` : list short names of all refs
- `git for-each-ref --sort -committerdate` : sort refs by committer date (most recent first)
- `git for-each-ref --sort committerdate` : sort refs by committer date (oldest first)
- `git for-each-ref --count count` : limit output to a specified number of refs

## Categories
version-control, packages

## Links
- More information: https://git-scm.com/docs/git-for-each-ref
- tldr page: https://tldr.inbrowser.app/pages/common/git-for-each-ref
- Wikipedia: https://en.wikipedia.org/wiki/Git
