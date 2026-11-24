# git submodule

Inspect, update, and manage submodules.

## Usage
- `git submodule` : view existing submodules and the checked-out commit for each one
- `git submodule update --init --recursive` : install a repository's submodules (listed in `.gitmodules`)
- `git submodule add repository_url` : add a Git repository as a submodule of the current one
- `git submodule add repository_url path/to/directory` : add a Git repository as a submodule of the current one, at a specific directory
- `git submodule update --remote` : update submodules to their latest commits
- `git submodule set-url path/to/submodule new_url` : change the URL of a submodule
- `git submodule deinit path/to/submodule` : unregister a submodule (e.g. before removing it from the repository with `git rm`)

## Categories
version-control, files, packages

## Links
- More information: https://git-scm.com/docs/git-submodule
- tldr page: https://tldr.inbrowser.app/pages/common/git-submodule
- Wikipedia: https://en.wikipedia.org/wiki/Git
