# git show-index

Show the packed archive index of a Git repository.

## Usage
- `git show-index path/to/file.idx` : read an IDX file for a Git packfile and dump its contents to `stdout`
- `git show-index --object-format sha1|sha256 path/to/file` : specify the hash algorithm for the index file (experimental)

## Categories
version-control, files, archives

## Links
- More information: https://git-scm.com/docs/git-show-index
- tldr page: https://tldr.inbrowser.app/pages/common/git-show-index
