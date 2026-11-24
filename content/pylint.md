# pylint

A Python code linter.

## Usage
- `pylint path/to/file.py` : show lint errors in a file
- `pylint package_or_module` : lint a package or module (must be importable; no `.py` suffix)
- `pylint path/to/directory` : lint a package from a directory path (must contain an `__init__.py` file)
- `pylint --rcfile path/to/pylintrc path/to/file.py` : lint a file and use a configuration file (usually named `pylintrc`)
- `pylint --disable C,W,no-error,design path/to/file` : lint a file and disable a specific error code

## Categories
files

## Links
- More information: https://pylint.pycqa.org/en/latest/
- tldr page: https://tldr.inbrowser.app/pages/common/pylint
- Wikipedia: https://en.wikipedia.org/wiki/Pylint
