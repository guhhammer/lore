# uv

uv is a very fast Python package and project manager from Astral. It replaces pip, venv, pipx and part of pyenv with one command.

## Tasks
### Projects
- `uv init NAME` : start a new project with a pyproject.toml
- `uv add requests` : add a dependency and install it
- `uv add --dev pytest` : add a development dependency
- `uv remove requests` : drop a dependency
- `uv sync` : make the environment match the lock file
- `uv lock` : write or refresh uv.lock
- `uv run script.py` : run a script inside the project environment
- `uv run pytest` : run a tool inside the project environment
- `uv tree` : show the dependency tree
### Python versions
- `uv python list` : show the Pythons available
- `uv python install 3.13` : download a Python version
- `uv python pin 3.12` : make the project use that version
- `uv venv --python 3.12` : make a venv with a chosen version
### Like pip
- `uv venv` : make .venv in the current folder
- `uv pip install requests` : install into the active or .venv environment
- `uv pip install -r requirements.txt` : install from a file
- `uv pip list` : show what is installed
- `uv pip freeze` : print pinned packages
- `uv pip compile requirements.in -o requirements.txt` : resolve and pin dependencies
### Tools and scripts
- `uv tool install ruff` : install a command line tool in its own environment
- `uvx ruff check .` : run a tool once without installing it
- `uv tool list` : show installed tools
- `uv tool upgrade --all` : update every tool
- `uv cache clean` : empty the cache (deletes cached downloads)

## Notes
- Commit `uv.lock` so everyone installs the same versions.
- `uv run` creates and updates `.venv` for you: no need to activate it.
- Install it from the official page, not from a pasted script: see the install guide in Links.
- Update uv itself with `uv self update` if you installed it with the standalone installer.

## See also
- pip
- python-venv
- pipx
- cargo

## Links
- Docs: https://docs.astral.sh/uv/
- Install: https://docs.astral.sh/uv/getting-started/installation/

## Categories
packages development
