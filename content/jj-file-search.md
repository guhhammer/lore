# jj file search

Search for content in files in a `jj` repository.

## Usage
- `jj file search -p "regex"` : search for files with lines matching a `regex` in the working copy
- `jj file search -p "glob:*pattern*"` : search for files with lines matching a `glob` pattern in the working copy
- `jj file search -r revision -p "pattern"` : search for content in files in a specific revision
- `jj file search -p "pattern" path/to/file_or_directory` : search only within specific paths or files

## Categories
files, text, packages

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-file-search
- tldr page: https://tldr.inbrowser.app/pages/common/jj-file-search
