# jj git clone

Create a new repo backed by a clone of a Git repo. Note: Unless `--colocate` is used, it is not a valid Git repository and `git` commands can't be used on it.

## Usage
- `jj git clone source` : create a new repo backed by a clone of a Git repo (the default directory is the repository name)
- `jj git clone source path/to/directory` : create a new repo backed by a clone of a Git repo in a specific directory
- `jj git clone --remote remote_name source` : create a clone and use the given name for newly created remote
- `jj git clone --depth 10 source` : clone a Git repo, only fetching the 10 most recent commits
- `jj git clone --colocate source` : clone colocating the Jujutsu repo with the Git repo (allowing the use of both `jj` and `git` commands in the same directory)

## Categories
version-control, files, packages

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-git-clone
- tldr page: https://tldr.inbrowser.app/pages/common/jj-git-clone
