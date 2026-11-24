# git restore

Restore working tree files. Requires Git version 2.23+.

## Usage
- `git restore path/to/file` : restore an unstaged file to the staged version
- `git restore -s commit path/to/file` : restore an unstaged file to the version of a specific commit
- `git restore :/` : discard all unstaged changes to tracked files
- `git restore -S path/to/file` : unstage a file
- `git restore -S :/` : unstage all files
- `git restore -W -S :/` : discard all changes to files, both staged and unstaged
- `git restore -p` : interactively select sections of files to restore

## See also
- git checkout
- git reset

## Categories
version-control, files

## Links
- More information: https://git-scm.com/docs/git-restore
- tldr page: https://tldr.inbrowser.app/pages/common/git-restore
- Wikipedia: https://en.wikipedia.org/wiki/Git
