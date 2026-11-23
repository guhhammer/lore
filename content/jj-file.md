# jj file

Manage and inspect files in a `jj` repository. Some subcommands such as `annotate`, `show`, `track`, `untrack`, etc. have their own usage documentation.

## Usage
- `jj file list` : list files tracked in the working copy
- `jj file show path/to/file` : print the contents of a file in the working copy
- `jj file show -r revset path/to/file` : print the contents of a file from another revision
- `jj file search -p "pattern"` : search for a pattern across tracked files
- `jj file annotate path/to/file` : show line-by-line source change annotation for a file
- `jj file track path/to/file_or_directory` : track specified paths in the working copy
- `jj file untrack path/to/file_or_directory` : stop tracking specified paths in the working copy
- `jj file chmod -x path/to/file` : set or remove the executable bit for a file

## Categories
files, documents, text

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-file
- tldr page: https://tldr.inbrowser.app/pages/common/jj-file
