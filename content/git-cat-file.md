# git cat-file

Provide content or type and size information for Git repository objects.

## Usage
- `git cat-file -s HEAD` : get the size of the `HEAD` commit in bytes
- `git cat-file -t 8c442dc3` : get the type (blob, tree, commit, tag) of a given Git object
- `git cat-file -p HEAD~2` : pretty-print the contents of a given Git object based on its type

## Categories
version-control, files, packages

## Links
- More information: https://git-scm.com/docs/git-cat-file
- tldr page: https://tldr.inbrowser.app/pages/common/git-cat-file
- Wikipedia: https://en.wikipedia.org/wiki/Git
