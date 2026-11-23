# uv remove

Remove dependencies from the project's `pyproject.toml` file.

## Usage
- `uv remove package` : remove a dependency from the project
- `uv remove package1 package2 ...` : remove multiple dependencies
- `uv remove --dev package` : remove a development dependency
- `uv remove --optional extra_name package` : remove a dependency from an optional dependency group
- `uv remove --group group_name package` : remove a dependency from a specific dependency group
- `uv remove --no-sync package` : remove without syncing the virtual environment

## Categories
packages, development, files

## Links
- More information: https://docs.astral.sh/uv/reference/cli/#uv-remove
- tldr page: https://tldr.inbrowser.app/pages/common/uv-remove
