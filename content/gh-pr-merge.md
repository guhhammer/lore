# gh pr merge

Merge a GitHub pull request.

## Usage
- `gh pr merge` : merge the pull request associated with the current branch interactively
- `gh pr merge pr_number -m` : merge the current branch into the specified pull request
- `gh pr merge pr_number -sd` : squash and merge a pull request, then delete the branch
- `gh pr merge pr_number -r` : rebase and merge
- `gh pr merge pr_number --auto -s` : enable auto-merge (squash)
- `gh pr merge pr_number --admin` : merge with admin privileges (if allowed)

## Categories
version-control

## Links
- More information: https://cli.github.com/manual/gh_pr_merge
- tldr page: https://tldr.inbrowser.app/pages/common/gh-pr-merge
