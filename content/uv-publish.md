# uv publish

Upload distributions to an index.

## Usage
- `uv publish` : publish packages from `dist/` directory (default behavior)
- `uv publish --publish-url https://upload.pypi.org/legacy/` : publish to a specific repository URL
- `uv publish -u username -p password` : publish using a specific username and password
- `uv publish -t your_api_token` : publish using an API token
- `uv publish path/to/dist/*.whl path/to/dist/*.tar.gz` : publish specific distribution files
- `uv publish --publish-url https://test.pypi.org/legacy/` : publish to TestPyPI for testing

## Categories
network, files

## Links
- More information: https://docs.astral.sh/uv/reference/cli/#uv-publish
- tldr page: https://tldr.inbrowser.app/pages/common/uv-publish
