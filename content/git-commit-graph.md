# git commit-graph

Write and verify Git commit-graph files.

## Usage
- `git commit-graph write` : write a commit-graph file for the packed commits in the repository's local `.git` directory
- `git show-ref -s | git commit-graph write --stdin-commits` : write a commit-graph file containing all reachable commits
- `git rev-parse HEAD | git commit-graph write --stdin-commits --append` : write a commit-graph file containing all commits in the current commit-graph file along with those reachable from `HEAD`

## Categories
version-control, files

## Links
- More information: https://git-scm.com/docs/git-commit-graph
- tldr page: https://tldr.inbrowser.app/pages/common/git-commit-graph
