# gh repo create

Create a new GitHub repository. Note: `--public`, `--private`, or `--internal` is required when not running interactively.

## Usage
- `gh repo new` : create a new repository interactively
- `gh repo new -s . --private` : create a private repository from the current directory
- `gh repo new -s . --public` : create a public repository from the current directory
- `gh repo new repo_name -d "repo_description" --public` : create a public repository with a specified name and description
- `gh repo new repo_name -c --public|--private|--internal` : clone the new repository locally after creation

## Categories
packages

## Links
- More information: https://cli.github.com/manual/gh_repo_create
- tldr page: https://tldr.inbrowser.app/pages/common/gh-repo-create
