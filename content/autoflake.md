# autoflake

Remove unused imports and variables from Python code.

## Usage
- `autoflake --remove-unused-variables path/to/file.py` : remove unused variables from a single file and display the diff
- `autoflake --remove-all-unused-imports path/to/file1.py path/to/file2.py ...` : remove unused imports from multiple files and display the diffs
- `autoflake --remove-unused-variables --in-place path/to/file.py` : remove unused variables from a file, overwriting the file
- `autoflake --remove-unused-variables --in-place --recursive path/to/directory` : remove unused variables recursively from all files in a directory, overwriting each file

## Categories
files

## Links
- More information: https://github.com/PyCQA/autoflake#advanced-usage
- tldr page: https://tldr.inbrowser.app/pages/common/autoflake
