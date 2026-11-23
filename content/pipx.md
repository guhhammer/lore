# pipx

Install and run Python applications in isolated environments.

## Usage
- `pipx run pycowsay moo` : run an app in a temporary virtual environment
- `pipx install package` : install a package in a virtual environment and add entry points to path
- `pipx uninstall package` : uninstall a package
- `pipx list` : list installed packages
- `pipx run --spec httpx-cli httpx http://www.github.com` : run an app in a temporary virtual environment with a package name different from the executable
- `pipx inject package dependency1 dependency2 ...` : inject dependencies into an existing virtual environment
- `pipx install --pip-args='pip-args' package` : install a package in a virtual environment with pip arguments
- `pipx upgrade-all|uninstall-all|reinstall-all` : upgrade/reinstall/uninstall all installed packages

## Categories
packages, network

## Links
- More information: https://manned.org/pipx
- tldr page: https://tldr.inbrowser.app/pages/common/pipx
