# uv build

Build Python packages into source distributions and wheels.

## Usage
- `uv build` : build a package in the current directory
- `uv build path/to/directory` : build a package from a specific directory
- `uv build --wheel` : build only a wheel (skip source distribution)
- `uv build --sdist` : build only a source distribution (skip wheel)
- `uv build -o path/to/output` : build and output to a specific directory
- `uv build --package package_name` : build a specific package in a workspace
- `uv build --all` : build all packages in the workspace
- `uv build -p python3.11` : build with a specific Python interpreter

## Categories
development, packages, files

## Links
- More information: https://docs.astral.sh/uv/reference/cli/#uv-build
- tldr page: https://tldr.inbrowser.app/pages/common/uv-build
