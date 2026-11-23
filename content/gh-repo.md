# gh repo

Work with GitHub repositories.

## Usage
- `gh repo new` : create a new repository interactively
- `gh repo clone owner/repository` : clone a repository
- `gh repo fork owner/repository --clone` : fork and clone a repository
- `gh repo view repository -w` : view a repository in the default web browser
- `gh repo ls owner` : list repositories owned by a specific user or organization (if the owner is not set, the default owner will be the currently logged in user)
- `gh repo ls owner --source -L limit` : list only non-forks repositories and limit the number of repositories to list (default: 30)
- `gh repo ls owner -l language_name` : list repositories with a specific primary coding language
- `gh repo sync` : sync the current local repository with its upstream

## Categories
packages

## Links
- More information: https://cli.github.com/manual/gh_repo
- tldr page: https://tldr.inbrowser.app/pages/common/gh-repo
