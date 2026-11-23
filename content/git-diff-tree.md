# git diff-tree

Compare the content and mode of blobs found via two tree objects.

## Usage
- `git diff-tree tree-ish1 tree-ish2` : compare two tree objects
- `git diff-tree -r commit1 commit2` : show changes between two specific commits
- `git diff-tree -p tree-ish1 tree-ish2` : display changes in patch format
- `git diff-tree tree-ish1 tree-ish2 -- path/to/file_or_directory` : filter changes by a specific path

## Categories
version-control

## Links
- More information: https://git-scm.com/docs/git-diff-tree
- tldr page: https://tldr.inbrowser.app/pages/common/git-diff-tree
