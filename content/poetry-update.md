# poetry update

Update the dependencies as according to the `pyproject.toml` file.

## Usage
- `poetry update` : update all dependencies
- `poetry update package1 package2 ...` : update one or more specific packages
- `poetry update --lock` : update the lock file only, without installing the packages
- `poetry update --sync` : synchronize the environment with the locked packages
- `poetry update --only group_name` : update dependencies only for a specific group
- `poetry update --dry-run` : simulate the update process without making changes

## Categories
packages, files

## Links
- More information: https://python-poetry.org/docs/cli/#update
- tldr page: https://tldr.inbrowser.app/pages/common/poetry-update
