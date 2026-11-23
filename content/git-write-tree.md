# git write-tree

Low level utility to create a tree object from the current index.

## Usage
- `git write-tree` : create a tree object from the current index
- `git write-tree --missing-ok` : create a tree object without checking whether objects referenced by the directory exist in the object database
- `git write-tree --prefix subdirectory/` : create a tree object that represents a subdirectory (used to write the tree object for a subproject in the named subdirectory)

## Categories
version-control

## Links
- More information: https://git-scm.com/docs/git-write-tree
- tldr page: https://tldr.inbrowser.app/pages/common/git-write-tree
