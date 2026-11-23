# poetry-self

Manage the Poetry installation/runtime environment itself. These commands reference `pyproject.toml` and `poetry.lock` files in your Poetry configuration directory.

## Usage
- `poetry self add package_name` : install a package
- `poetry self install` : install dependencies from the Poetry installation's `pyproject.toml` file
- `poetry self lock` : lock dependencies from the Poetry installation's `pyproject.toml` file
- `poetry self remove package_name` : remove a package
- `poetry self show` : list all installed packages
- `poetry self show plugins` : list all installed plugins
- `poetry self sync` : sync the runtime environment with the Poetry installation's `poetry.lock` file
- `poetry self update` : update dependencies from the Poetry installation's `pyproject.toml` file

## See also
- asdf

## Categories
packages, files

## Links
- More information: https://python-poetry.org/docs/cli/#self
- tldr page: https://tldr.inbrowser.app/pages/common/poetry-self
