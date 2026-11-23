# jj git init

Create a new Git backed Jujutsu repo. Note: Unless `--colocate` is used, it is not a valid Git repository and `git` commands can't be used on it.

## Usage
- `jj git init` : create a new Git backed repo in the current directory
- `jj git init path/to/directory` : create a new Git backed repo in the given directory
- `jj git init --colocate` : initialize the Jujutsu repository as a valid Git repository (allowing the use of both `jj` and `git` commands in the same directory)
- `jj git init --git-repo git_repo` : initialize the Jujutsu repository backed by an existing Git repository

## Categories
version-control, packages, files

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-git-init
- tldr page: https://tldr.inbrowser.app/pages/common/jj-git-init
