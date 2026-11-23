# poetry sync

Sync your project's environment with the `poetry.lock` file.

## Usage
- `poetry sync` : sync your project's environment with the `poetry.lock` file
- `poetry sync --without test|docs|...` : exclude one or more dependency groups for the installation
- `poetry sync --with test|docs|...` : select optional dependency groups
- `poetry sync --all-groups` : install all dependency groups including optional groups
- `poetry sync --only test|docs|...` : install specific dependency groups
- `poetry sync --only-root` : install project without dependencies
- `poetry sync -E` : specify extras to install
- `poetry sync --no-root` : skip the defaulted package installation for your project

## Categories
development, packages, users

## Links
- More information: https://python-poetry.org/docs/cli/#sync
- tldr page: https://tldr.inbrowser.app/pages/common/poetry-sync
