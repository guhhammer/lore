# gh pr

Manage GitHub pull requests. Some subcommands such as `create` have their own usage documentation.

## Usage
- `gh pr new` : create a pull request
- `gh co pr_number|url|branch` : check out a specific pull request locally
- `gh pr diff` : view the changes made in the pull request for the current branch
- `gh pr review -a` : approve the pull request for the current branch
- `gh pr merge` : merge the pull request associated with the current branch interactively
- `gh pr edit` : edit a pull request interactively
- `gh pr edit -B branch_name` : edit the base branch of a pull request
- `gh pr status` : check the status of the current repository's pull requests

## Categories
version-control, documents, monitoring

## Links
- More information: https://cli.github.com/manual/gh_pr
- tldr page: https://tldr.inbrowser.app/pages/common/gh-pr
