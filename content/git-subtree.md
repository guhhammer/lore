# git subtree

Merge subtrees together or split repository into subtrees.

## Usage
- `git subtree add -P path/to/directory --squash repository_url branch_name` : add a Git repository as a subtree and squash the commits together
- `git subtree pull -P path/to/directory repository_url branch_name` : update subtree repository to its latest commit
- `git subtree merge -P path/to/directory --squash repository_url branch_name` : merge recent changes up to the latest subtree commit into the subtree
- `git subtree push -P path/to/directory repository_url branch_name` : push commits to a subtree repository
- `git subtree split -P path/to/directory repository_url -b branch_name` : extract a new project history from the history of a subtree

## Categories
version-control, files, packages

## Links
- More information: https://manned.org/git-subtree
- tldr page: https://tldr.inbrowser.app/pages/common/git-subtree
