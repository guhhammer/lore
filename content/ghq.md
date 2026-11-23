# ghq

Manage remote repository clones organized by hostname and path.

## Usage
- `ghq get repository_url` : clone a repository under the `ghq` root directory (default is `~/ghq`)
- `ghq get user/project` : clone a repository from a user/project format (defaults to GitHub)
- `ghq get repository_url --look` : clone a repository and `cd` into it
- `ghq get user/project -p` : clone a repository via SSH
- `ghq get repository_url -u` : update an existing repository to the latest version
- `ghq list` : list all locally cloned repositories
- `ghq list --full-path` : list locally cloned repositories with full paths
- `ghq rm user/project` : remove a locally cloned repository

## Categories
packages, files, development

## Links
- More information: https://github.com/x-motemen/ghq
- tldr page: https://tldr.inbrowser.app/pages/common/ghq
