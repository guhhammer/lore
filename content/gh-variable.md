# gh variable

Manage GitHub Actions and Dependabot variables.

## Usage
- `gh variable ls` : list variables for the current repository
- `gh variable ls -o organization` : list variables for a specific organization
- `gh variable get name` : get a variable for the current repository
- `gh variable set name` : set a variable for the current repository (user will be prompted for the value)
- `gh variable set name -e environment_name` : set a variable for a deployment environment in the current repository
- `gh variable set name -o organization -v all` : set an organization variable visible to all repositories
- `gh variable set -f path/to/file.env` : set multiple variables from a dotenv file
- `gh variable delete name` : delete a variable for the current repository

## Categories
packages, files

## Links
- More information: https://cli.github.com/manual/gh_variable
- tldr page: https://tldr.inbrowser.app/pages/common/gh-variable
