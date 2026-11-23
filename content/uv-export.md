# uv export

Export the project's lockfile to an alternate format.

## Usage
- `uv export --format requirements-txt -o requirements.txt` : export dependencies to a `requirements.txt` file
- `uv export --format pylock.toml` : export dependencies to `pylock.toml` format
- `uv export --no-dev` : export only production dependencies (exclude dev dependencies)
- `uv export --extra group_name` : export including a specific optional dependency group
- `uv export --all-extras` : export including all optional dependencies
- `uv export --group group_name` : export including a specific dependency group
- `uv export --no-hashes` : export without hashes
- `uv export --package package_name` : export dependencies for a specific package in the workspace

## Categories
packages, development, users

## Links
- More information: https://docs.astral.sh/uv/reference/cli/#uv-export
- tldr page: https://tldr.inbrowser.app/pages/common/uv-export
