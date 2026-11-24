# git checkout

Check out a branch or paths to the working tree.

## Usage
- `git checkout -b branch_name` : create and switch to a new branch
- `git checkout -b branch_name reference` : create and switch to a new branch based on a specific reference (branch, remote/branch, tag are examples of valid references)
- `git checkout branch_name` : switch to an existing local branch
- `git checkout -` : switch to the previously checked out branch
- `git checkout -t remote_name/branch_name` : switch to an existing remote branch
- `git checkout .` : discard all unstaged changes in the current directory (see `git reset` for more undo-like commands)
- `git checkout path/to/file` : discard unstaged changes to a given file
- `git checkout branch_name -- path/to/file` : replace a file in the current directory with the version of it committed in a given branch

## Categories
version-control, files

## Links
- More information: https://git-scm.com/docs/git-checkout
- tldr page: https://tldr.inbrowser.app/pages/common/git-checkout
- Wikipedia: https://en.wikipedia.org/wiki/Git
