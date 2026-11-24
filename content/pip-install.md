# pip install

Install Python packages.

## Usage
- `pip install package1 package2 ...` : install one or more packages
- `pip install package1 package2 ... -U` : upgrade all specified packages to the latest version, installing any that are not already present
- `pip install package==version` : install a specific version of a package
- `pip install -r path/to/requirements.txt` : install packages listed in a file
- `pip install path/to/file.whl|path/to/file.tar.gz|path/to/directory` : install a package from a local archive or directory
- `pip install git+https://example.com/user/repository.git` : install a package from a Git repository
- `pip install -f url|path/to/directory package` : install a package from an alternative source (URL or directory) instead of PyPI
- `pip install -e .` : install the local package in the current directory in develop (editable) mode

## Categories
packages, files, network

## Links
- More information: https://pip.pypa.io/en/stable/cli/pip_install/
- tldr page: https://tldr.inbrowser.app/pages/common/pip-install
- Wikipedia: https://en.wikipedia.org/wiki/Pip_(package_manager)
