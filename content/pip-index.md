# pip index

Inspect information available from package indexes.

## Usage
- `pip index versions package` : list all available versions of a package
- `pip index versions package --index-url https://test.pypi.org/simple/` : list versions from a specific index
- `pip index versions package --pre` : include pre-release versions
- `pip index versions package --extra-index-url https://example.com/simple/` : include an additional index
- `pip index versions package --platform linux_x86_64` : list versions for a specific platform

## Categories
packages, network

## Links
- More information: https://pip.pypa.io/en/stable/cli/pip_index/
- tldr page: https://tldr.inbrowser.app/pages/common/pip-index
