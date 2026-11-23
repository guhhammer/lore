# diff

Compare files and directories.

## Usage
- `diff path/to/old_file path/to/new_file` : compare files (lists changes to turn `old_file` into `new_file`)
- `diff -w path/to/old_file path/to/new_file` : compare files, ignoring white spaces
- `diff -y path/to/old_file path/to/new_file` : compare files, showing the differences side by side
- `diff -u path/to/old_file path/to/new_file` : compare files, showing the differences in unified format (as used by `git diff`)
- `diff -r path/to/old_directory path/to/new_directory` : compare directories recursively (shows names for differing files/directories as well as changes made to files)
- `diff -r -q path/to/old_directory path/to/new_directory` : compare directories, only showing the names of files that differ
- `diff -a -u -N path/to/old_file path/to/new_file >path/to/diff.patch` : create a patch file for Git from the differences of two text files, treating nonexistent files as empty
- `diff -d --color=always path/to/old_file path/to/new_file` : compare files, showing output in color, trying hard to find the smallest set of changes

## See also
- delta
- difft

## Categories
text, version-control

## Links
- More information: https://manned.org/diff
- tldr page: https://tldr.inbrowser.app/pages/common/diff
