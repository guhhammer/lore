# git mktree

Build a tree object using `ls-tree` formatted text.

## Usage
- `git mktree` : build a tree object and verify that each tree entry's hash identifies an existing object
- `git mktree --missing` : allow missing objects
- `git mktree -z` : read the NUL (zero character) terminated output of the tree object (`git ls-tree -z`)
- `git mktree --batch` : allow the creation of multiple tree objects
- `git <path/to/tree.txt mktree` : sort and build a tree from `stdin` (non-recursive `git ls-tree` output format is required)

## Categories
version-control, development, text

## Links
- More information: https://git-scm.com/docs/git-mktree
- tldr page: https://tldr.inbrowser.app/pages/common/git-mktree
- Wikipedia: https://en.wikipedia.org/wiki/Git
