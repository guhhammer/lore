# ruff format

An extremely fast Python code formatter. If no files or directories are specified, the current working directory is used by default.

## Usage
- `ruff format path/to/file_or_directory1 path/to/file_or_directory2 ...` : format given files or directories in-place
- `ruff format --check` : print which files would have been modified and return a non-zero exit code if there are files to reformat, and zero otherwise
- `ruff format --diff` : print what changes would be made without modifying the files

## Categories
files

## Links
- More information: https://docs.astral.sh/ruff/formatter/
- tldr page: https://tldr.inbrowser.app/pages/common/ruff-format
