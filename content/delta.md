# delta

A viewer for Git and diff output.

## Usage
- `delta path/to/old_file_or_directory path/to/new_file_or_directory` : compare files or directories
- `delta -n path/to/old_file_or_directory path/to/new_file_or_directory` : compare files or directories, showing the line numbers
- `delta -s path/to/old_file_or_directory path/to/new_file_or_directory` : compare files or directories, showing the differences side by side
- `delta --no-gitconfig path/to/old_file_or_directory path/to/new_file_or_directory` : compare files or directories, ignoring any Git configuration settings
- `delta --hyperlinks path/to/old_file_or_directory path/to/new_file_or_directory` : compare, rendering commit hashes, file names, and line numbers as hyperlinks, according to the hyperlink spec for terminal emulators
- `delta --show-config` : display the current settings
- `delta --list-languages` : display supported languages and associated file extensions

## See also
- diff
- difft

## Categories
files, version-control

## Links
- More information: https://dandavison.github.io/delta/full---help-output.html
- tldr page: https://tldr.inbrowser.app/pages/common/delta
