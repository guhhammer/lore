# uv run

Run a command or script in the project environment.

## Usage
- `uv run path/to/script.py` : run a Python script
- `uv run -m module_name` : run a Python module
- `uv run -w package command` : run a command with additional packages installed temporarily
- `uv run --with-requirements path/to/requirements.txt path/to/script.py` : run a script with packages from a requirements file
- `uv run --isolated path/to/script.py` : run in an isolated environment (no project dependencies)
- `uv run --no-sync command` : run without syncing the environment first

## Categories
development, files, packages

## Links
- More information: https://docs.astral.sh/uv/reference/cli/#uv-run
- tldr page: https://tldr.inbrowser.app/pages/common/uv-run
