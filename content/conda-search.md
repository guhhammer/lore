# conda search

Search for packages and show their details.

## Usage
- `conda search package_name` : search for a specific package
- `conda search package_name -i` : search for a package along with its details
- `conda search "*string*"` : search for packages containing `string` in the package name
- `conda search "package_name>=package_version"` : search for specific version of the package
- `conda search channel::package_name` : search a package within a specific channel
- `conda search --envs package_name` : search if package is installed in any local environments

## Categories
packages

## Links
- More information: https://docs.conda.io/projects/conda/en/latest/commands/search.html
- tldr page: https://tldr.inbrowser.app/pages/common/conda-search
