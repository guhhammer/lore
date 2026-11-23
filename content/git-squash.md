# git squash

Squash multiple commits into a single commit. Part of `git-extras`.

## Usage
- `git squash source_branch` : merge all commits from a specific branch into the current branch as a single commit
- `git squash commit` : squash all commits starting with a specific commit on the current branch
- `git squash HEAD~n "message"` : squash the `n` latest commits and commit with a message
- `git squash --squash-msg HEAD~n` : squash the `n` latest commits and commit concatenating all individual messages

## Categories
version-control

## Links
- More information: https://manned.org/git-squash
- tldr page: https://tldr.inbrowser.app/pages/common/git-squash
