# py

Python Launcher for Windows that runs with specified Python version.

## Usage
- `py python_arguments` : start a REPL (interactive shell), optionally with arguments supported by `python` (like `-c`, `-m`, etc.)
- `py path\to\file.py` : execute a specific Python file
- `py -2|-3.7|...` : run specific Python version. If the version is missing, and `PYLAUNCHER_ALLOW_INSTALL` environment variable is set, auto-install via Microsoft Store or Winget
- `py --list` : list installed Python versions

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## See also
- python

## Categories
files, windows

## Links
- More information: https://docs.python.org/using/windows.html#python-launcher-for-windows
- tldr page: https://tldr.inbrowser.app/pages/windows/py
