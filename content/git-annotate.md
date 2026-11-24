# git annotate

Show commit hash and last author on each line of a file. See `git blame`, which is preferred over `git annotate`. `git annotate` is provided for those familiar with other version control systems.

## Usage
- `git annotate path/to/file` : print a file with the author name and commit hash prepended to each line
- `git annotate -e path/to/file` : print a file with the author email and commit hash prepended to each line
- `git annotate -L :regex path/to/file` : print only rows that match a `regex`

## Categories
version-control, files, text

## Links
- More information: https://git-scm.com/docs/git-annotate
- tldr page: https://tldr.inbrowser.app/pages/common/git-annotate
- Wikipedia: https://en.wikipedia.org/wiki/Git
