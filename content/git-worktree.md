# git worktree

Manage multiple working trees attached to the same repository.

## Usage
- `git worktree add branch` : add a new worktree directory for a branch with the same name (created if missing)
- `git worktree add path/to/directory branch` : create a new directory with the specified branch checked out into it
- `git worktree add path/to/directory -b new_branch` : create a new directory with a new branch checked out into it
- `git worktree list` : list all the working directories (including the primary one)
- `git worktree move path/to/worktree path/to/new_worktree` : move an existing worktree to a new location
- `git worktree remove path/to/worktree` : remove a worktree directory and its metadata (only if it has no uncommitted changes)
- `git worktree prune` : remove metadata for a worktree directory that's been manually deleted

## Categories
version-control, files, packages

## Links
- More information: https://git-scm.com/docs/git-worktree
- tldr page: https://tldr.inbrowser.app/pages/common/git-worktree
