# uv lock

Update the project's lockfile.

## Usage
- `uv lock` : create or update the project's lockfile
- `uv lock --check` : check if the lockfile is up-to-date without updating it
- `uv lock --check-exists` : assert that a lockfile exists without checking if it's current
- `uv lock --dry-run` : simulate locking, showing what would be locked without writing the lockfile
- `uv lock --script path/to/script.py` : lock a specific Python script instead of the current project
- `uv lock --upgrade` : upgrade all packages to their latest compatible versions
- `uv lock --upgrade-package package1 --upgrade-package package2` : upgrade only specific packages

## Categories
packages, development

## Links
- More information: https://docs.astral.sh/uv/reference/cli/#uv-lock
- tldr page: https://tldr.inbrowser.app/pages/common/uv-lock
