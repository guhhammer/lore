# poetry export

Export Poetry's lock file to other formats. Provided by the Export Poetry Plugin.

## Usage
- `poetry export -o requirements.txt` : export dependencies to a `requirements.txt` file
- `poetry export -o requirements-dev.txt --dev` : export dependencies including development dependencies
- `poetry export -o requirements.txt --without-hashes` : export dependencies without hashes
- `poetry export -o requirements.txt -f requirements.txt` : export dependencies for a specific format
- `poetry export -o requirements.txt --only main` : export only specific dependency groups
- `poetry export -h` : display help

## Categories
packages, files

## Links
- More information: https://github.com/python-poetry/poetry-plugin-export#usage
- tldr page: https://tldr.inbrowser.app/pages/common/poetry-export
