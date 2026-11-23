# poetry-env

Manage virtual environments associated with a Poetry project.

## Usage
- `poetry env activate` : print the command to activate a virtual environment
- `poetry env info` : display information about the current environment
- `poetry env info -p` : display the path of the current environment
- `poetry env info -e` : display the path of the current environment's Python executable
- `poetry env list --full-path` : list all virtual environments associated with the current project (optionally showing the full path)
- `poetry env remove python path/to/executable|environment_name | poetry env remove --all` : remove specific or all virtual environments associated with the current project
- `poetry env use python path/to/executable` : activate or create a virtual environment for the project using the specified Python executable

## See also
- asdf

## Categories
development, files, desktop

## Links
- More information: https://python-poetry.org/docs/cli/#env
- tldr page: https://tldr.inbrowser.app/pages/common/poetry-env
