# poetry cache

Manage Poetry's cache.

## Usage
- `poetry cache list` : display Poetry's available caches
- `poetry cache clear PyPI --all` : remove all packages from a cache (e.g. PyPI)
- `poetry cache clear pypi:requests:2.24.0` : remove a specific package from a cache (Note: must be in format `cache:package:version`)

## See also
- asdf

## Categories
packages

## Links
- More information: https://python-poetry.org/docs/cli/#cache
- tldr page: https://tldr.inbrowser.app/pages/common/poetry-cache
