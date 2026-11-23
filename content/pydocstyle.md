# pydocstyle

Statically check Python scripts for compliance with Python docstring conventions.

## Usage
- `pydocstyle file.py|path/to/directory` : analyze a Python script or all the Python scripts in a specific directory
- `pydocstyle -e file.py|path/to/directory` : show an explanation of each error
- `pydocstyle -d file.py|path/to/directory` : show debug information
- `pydocstyle --count file.py|path/to/directory` : display the total number of errors
- `pydocstyle --config path/to/config_file file.py|path/to/directory` : use a specific configuration file
- `pydocstyle --ignore D101,D2,D107,... file.py|path/to/directory` : ignore one or more errors
- `pydocstyle --convention pep257|numpy|google file.py|path/to/directory` : check for errors from a specific convention

## Categories
files

## Links
- More information: https://pydocstyle.readthedocs.io/en/latest/usage.html
- tldr page: https://tldr.inbrowser.app/pages/common/pydocstyle
