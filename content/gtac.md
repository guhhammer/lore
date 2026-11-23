# gtac

This command is an alias of GNU `tac`.

## Usage
- `tac path/to/file1 path/to/file2 ...` : concatenate specific files in reversed order
- `cat path/to/file | tac` : display `stdin` in reversed order
- `tac -s separator path/to/file1 path/to/file2 ...` : use a specific separator
- `tac -r -s separator path/to/file1 path/to/file2 ...` : use a specific `regex` as a separator
- `tac -b path/to/file1 path/to/file2 ...` : use a separator before each file

## Categories
files, shell, macos

## Links
- tldr page: https://tldr.inbrowser.app/pages/osx/gtac
