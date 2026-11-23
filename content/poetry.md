# poetry

Manage Python packages and dependencies. Some subcommands such as `about`, `check`, `env`, etc. have their own usage documentation.

## Usage
- `poetry new project_name` : create a new Poetry project in the directory with a specific name
- `poetry add dependency` : install and add a dependency and its sub-dependencies to the `pyproject.toml` file in the current directory
- `poetry install` : install the project dependencies using the `pyproject.toml` file in the current directory
- `poetry init` : interactively (append `-n` for non-interactively) initialize the current directory as a new Poetry project
- `poetry update` : get the latest version of all dependencies and update `poetry.lock`
- `poetry run command` : execute a command inside the project's virtual environment
- `poetry version patch|minor|major|prepatch|preminor|premajor|prerelease` : bump the version of the project in `pyproject.toml`
- `eval "$(poetry env activate)"` : spawn a shell within the project's virtual environment (for versions below 2.0, use `poetry shell`)

## See also
- asdf
- pipenv
- hatch

## Categories
packages, development, files

## Links
- More information: https://python-poetry.org/docs/cli/
- tldr page: https://tldr.inbrowser.app/pages/common/poetry
