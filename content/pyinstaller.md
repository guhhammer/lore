# pyinstaller

Bundle a Python application and its dependencies into a single package.

## Usage
- `pyinstaller path/to/script.py` : bundle a Python script into a one-folder bundle (output goes to `dist/`)
- `pyinstaller -F path/to/script.py` : bundle a Python script into a single executable file
- `pyinstaller -w path/to/script.py` : bundle a GUI application without opening a console window
- `pyinstaller -n app_name path/to/script.py` : set the name of the bundled executable
- `pyinstaller -i path/to/icon.ico path/to/script.py` : set a custom icon for the executable
- `pyinstaller --add-data "path/to/source:destination" path/to/script.py` : include additional data files or directories in the bundle
- `pyinstaller -h` : display help
- `pyinstaller -v` : display version

## Categories
files, development, packages

## Links
- More information: https://pyinstaller.org/en/stable/man/pyinstaller.html
- tldr page: https://tldr.inbrowser.app/pages/common/pyinstaller
