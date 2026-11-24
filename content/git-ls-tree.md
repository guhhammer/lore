# git ls-tree

List the contents of a tree object.

## Usage
- `git ls-tree branch_name` : list the contents of the tree on a branch
- `git ls-tree -r commit_hash` : list the contents of the tree on a commit, recursing into subtrees
- `git ls-tree --name-only commit_hash` : list only the filenames of the tree on a commit
- `git ls-tree -r --name-only HEAD | tree --fromfile` : print the filenames of the current branch head in a tree structure (Note: `tree --fromfile` is not supported on Windows)

## Categories
version-control

## Links
- More information: https://git-scm.com/docs/git-ls-tree
- tldr page: https://tldr.inbrowser.app/pages/common/git-ls-tree
- Wikipedia: https://en.wikipedia.org/wiki/Git
