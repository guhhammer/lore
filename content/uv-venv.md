# uv venv

Create an isolated Python environment for installing packages.

## Usage
- `uv venv` : create a virtual environment in the default location (`.venv`)
- `uv venv path/to/venv` : create a virtual environment at a specific path
- `uv venv -p 3.12` : create using a specific Python version
- `uv venv --seed` : include seed packages such as `pip` when creating the virtual environment
- `uv venv --prompt my_project` : create with a custom prompt prefix
- `uv venv --allow-existing venv_name` : create and allow overwriting existing environment

## Categories
packages

## Links
- More information: https://docs.astral.sh/uv/reference/cli/#uv-venv
- tldr page: https://tldr.inbrowser.app/pages/common/uv-venv
