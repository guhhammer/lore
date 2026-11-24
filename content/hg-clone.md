# hg clone

Create a copy of an existing repository in a new directory.

## Usage
- `hg clone remote_repository_source path/to/destination` : clone a repository to a specified directory
- `hg clone -b branch remote_repository_source` : clone a repository to the head of a specific branch, ignoring later commits
- `hg clone -U remote_repository_source` : clone a repository with only the `.hg` directory, without checking out files
- `hg clone -u revision remote_repository_source` : clone a repository to a specific revision, tag, or branch, keeping the entire history
- `hg clone -r revision remote_repository_source` : clone a repository up to a specific revision without any newer history

## Categories
files, packages, version-control

## Links
- More information: https://www.mercurial-scm.org/help/commands/clone
- tldr page: https://tldr.inbrowser.app/pages/common/hg-clone
- Wikipedia: https://en.wikipedia.org/wiki/Mercurial
