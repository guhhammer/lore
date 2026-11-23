# gomi

Manage the trashcan.

## Usage
- `gomi path/to/file1 path/to/file2 path/to/folder1 path/to/folder2 ...` : safely delete specific files or folders
- `gomi -b` : open an interactive menu to restore one or more files
- `gomi --prune 1d|1w|1m|1y|...` : remove files that have been in the trashcan longer than the specified time (day, week, month, year)
- `gomi --prune orphans` : remove orphaned `.trashinfo` files

## See also
- trash
- rm

## Categories
files

## Links
- More information: https://github.com/babarot/gomi
- tldr page: https://tldr.inbrowser.app/pages/common/gomi
