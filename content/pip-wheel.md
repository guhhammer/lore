# pip wheel

Build wheel archives for packages and dependencies.

## Usage
- `pip wheel package` : build a wheel for a package
- `pip wheel -r path/to/requirements.txt` : build wheels for packages in requirements file
- `pip wheel package -w path/to/directory` : build wheel to a specific directory
- `pip wheel package --no-deps` : build wheel without dependencies
- `pip wheel path/to/project` : build wheel from local project
- `pip wheel git+https://github.com/user/repo.git` : build wheel from Git repository

## Categories
packages, development, files

## Links
- More information: https://pip.pypa.io/en/stable/cli/pip_wheel/
- tldr page: https://tldr.inbrowser.app/pages/common/pip-wheel
