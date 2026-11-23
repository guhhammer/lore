# gh stack

A GitHub CLI extension for managing stacked branches and pull requests.

## Usage
- `gh stack init` : start a new stack interactively (creates and checks out the first branch)
- `gh stack init branch_name1 branch_name2 ...` : start a new stack specifying branches upfront
- `gh stack add branch_name` : add a branch on top of the stack
- `gh stack push` : push all branches to the default remote
- `gh stack push --remote remote_name` : push all branches to a specified remote
- `gh stack view` : view the stack
- `gh stack submit` : open a stack of PRs

## Links
- More information: https://docs.github.com/en/pull-requests/reference/stacked-prs-cli-commands
- tldr page: https://tldr.inbrowser.app/pages/common/gh-stack
