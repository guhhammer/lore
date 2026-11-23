# jj git

Run Git-related commands for a `jj` repository.

## Usage
- `jj git init` : create a new Git backed repository
- `jj git clone source` : create a new repository backed by a clone of a Git repository
- `jj git clone --no-colocate source` : clone a Git repository as a `jj` repository, without colocation
- `jj git fetch` : fetch from a Git remote
- `jj git push` : push all tracked bookmarks to Git remote
- `jj git push -b bookmark` : push given bookmark to Git remote
- `jj git colocation enable` : convert into a colocated Jujutsu/Git repository
- `jj git root` : show the path to the underlying Git directory

## Categories
version-control, packages

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-git
- tldr page: https://tldr.inbrowser.app/pages/common/jj-git
