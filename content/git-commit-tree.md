# git commit-tree

Low level utility to create commit objects.

## Usage
- `git commit-tree tree -m "message"` : create a commit object with the specified message
- `git commit-tree tree -F path/to/file` : create a commit object reading the message from a file (use `-` for `stdin`)
- `git commit-tree tree -m "message" -S` : create a GPG-signed commit object
- `git commit-tree tree -m "message" -p parent_commit_sha` : create a commit object with the specified parent commit object

## See also
- git commit

## Categories
version-control, files

## Links
- More information: https://git-scm.com/docs/git-commit-tree
- tldr page: https://tldr.inbrowser.app/pages/common/git-commit-tree
- Wikipedia: https://en.wikipedia.org/wiki/Git
