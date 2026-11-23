# poetry remove

Remove a package from the project dependencies.

## Usage
- `poetry remove package1 package2 ...` : remove one or more packages from the project's dependencies
- `poetry remove package -D` : remove a package from the development dependencies
- `poetry remove package -G group_name` : remove a package from a specific dependency group
- `poetry remove package --dry-run` : simulate removing a package without making any changes
- `poetry remove package --lock` : update the lock file only, without removing the package from the environment

## Categories
packages, development

## Links
- More information: https://python-poetry.org/docs/cli/#remove
- tldr page: https://tldr.inbrowser.app/pages/common/poetry-remove
