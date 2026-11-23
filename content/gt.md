# gt

Create and manage sequences of dependent code changes (stacks) for Git and GitHub.

## Usage
- `gt init` : initialize `gt` for the repository in the current directory
- `gt create branch_name` : create a new branch stacked on top of the current branch and commit staged changes
- `gt modify -cam commit_message` : create a new commit and fix upstack branches
- `gt stack submit` : force push all branches in the current stack to GitHub and create or update PRs
- `gt co branch_name` : check out different branch (prompts interactive mode when branch name is omitted)
- `gt sync` : sync stack with remote version (also deletes merged branches)
- `gt log short` : log all tracked stacks
- `gt subcommand --help` : display help for a specified subcommand

## Categories
version-control

## Links
- More information: https://graphite.com/docs/get-started
- tldr page: https://tldr.inbrowser.app/pages/common/gt
