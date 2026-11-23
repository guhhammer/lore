# vcsh

Version Control System for the home directory using Git repositories.

## Usage
- `vcsh init repository_name` : initialize an (empty) repository
- `vcsh clone git_url repository_name` : clone a repository into a custom directory name
- `vcsh list` : list all managed repositories
- `vcsh repository_name git_command` : execute a Git command on a managed repository
- `vcsh push|pull` : push/pull all managed repositories to/from remotes
- `vcsh write-gitignore repository_name` : write a custom `.gitignore` file for a managed repository

## See also
- chezmoi
- stow
- tuckr
- homeshick

## Categories
version-control, files, packages

## Links
- More information: https://manned.org/vcsh
- tldr page: https://tldr.inbrowser.app/pages/common/vcsh
