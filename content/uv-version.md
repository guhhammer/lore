# uv version

Read or update a project's version.

## Usage
- `uv version` : display the current project version
- `uv version 1.2.3` : set the project version to a specific value
- `uv version --bump major|minor|patch` : bump the project version using semantic versioning
- `uv version --bump patch --dry-run` : simulate a version bump without writing to `pyproject.toml`
- `uv version --package package_name 1.2.3` : update version for a specific package in a workspace
- `uv version --output-format json` : display version in JSON format

## Categories
development

## Links
- More information: https://docs.astral.sh/uv/reference/cli/#uv-version
- tldr page: https://tldr.inbrowser.app/pages/common/uv-version
