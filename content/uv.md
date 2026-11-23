# uv

A fast Python package and project manager. Some subcommands such as `tool`, `python`, etc. have their own usage documentation.

## Usage
- `uv init` : create a new Python project in the current directory
- `uv init path/to/directory` : create a new Python project at the specified path
- `uv add package` : add a new dependency to the project
- `uv remove package` : remove a dependency from the project
- `uv run path/to/script.py|command` : run a script or a command in the project's environment
- `uv sync` : update a project's environment from `pyproject.toml`
- `uv lock` : create a lock file for the project's dependencies
- `uv build` : build the project into source and binary distributions

## Categories
development, files, packages

## Links
- More information: https://docs.astral.sh/uv/reference/cli/
- tldr page: https://tldr.inbrowser.app/pages/common/uv
