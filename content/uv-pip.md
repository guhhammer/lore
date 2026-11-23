# uv pip

Provide pip-like commands for installing, uninstalling, and managing packages.

## Usage
- `uv pip install package` : install a package
- `uv pip install -r requirements.txt` : install packages from a requirements file
- `uv pip install package==1.2.3` : install a package with a specific version
- `uv pip uninstall package` : uninstall a package
- `uv pip compile pyproject.toml -o requirements.txt` : lock dependencies from `pyproject.toml` to `requirements.txt`
- `uv pip list` : list installed packages
- `uv pip show package` : show information about an installed package
- `uv pip sync requirements.txt` : sync environment with a requirements file (install/uninstall to match exactly)

## Categories
packages

## Links
- More information: https://docs.astral.sh/uv/reference/cli/#uv-pip
- tldr page: https://tldr.inbrowser.app/pages/common/uv-pip
