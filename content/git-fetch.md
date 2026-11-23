# git fetch

Download objects and refs from a remote repository.

## Usage
- `git fetch` : fetch the latest changes from the default remote upstream repository (if set)
- `git fetch remote_name` : fetch new branches from a specific remote upstream repository
- `git fetch --all` : fetch the latest changes from all remote upstream repositories
- `git fetch -t` : also fetch tags from the remote upstream repository
- `git fetch -p` : delete local references to remote branches that have been deleted upstream
- `git fetch --deepen 2` : deepen current shallow branch by 2 commits
- `git fetch origin main:main` : update the `main` branch without switching to it (equivalent to `git pull`)

## Categories
version-control, packages, network

## Links
- More information: https://git-scm.com/docs/git-fetch
- tldr page: https://tldr.inbrowser.app/pages/common/git-fetch
