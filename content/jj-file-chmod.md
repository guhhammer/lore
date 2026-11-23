# jj file chmod

Set or remove the executable bit for paths in a `jj` repository.

## Usage
- `jj file chmod x path/to/file` : make a file executable in the working copy
- `jj file chmod n path/to/file` : make a file non-executable (normal) in the working copy
- `jj file chmod x path/to/file1 path/to/file2 ...` : make multiple files executable
- `jj file chmod -r revision x path/to/file` : make a file executable in a specific revision

## Categories
files, packages

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-file-chmod
- tldr page: https://tldr.inbrowser.app/pages/common/jj-file-chmod
