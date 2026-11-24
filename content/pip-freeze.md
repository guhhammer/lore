# pip freeze

List installed packages in requirements format.

## Usage
- `pip freeze` : list installed packages
- `pip freeze >requirements.txt` : write installed packages to the `requirements.txt` file
- `pip freeze -l` : list installed packages in a virtual environment, excluding globally installed packages
- `pip freeze --user` : list installed packages in the user-site
- `pip freeze --all` : list all packages, including `pip`, `distribute`, `setuptools`, and `wheel` (they are skipped by default)

## Categories
packages

## Links
- More information: https://pip.pypa.io/en/stable/cli/pip_freeze/
- tldr page: https://tldr.inbrowser.app/pages/common/pip-freeze
- Wikipedia: https://en.wikipedia.org/wiki/Pip_(package_manager)
