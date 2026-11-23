# attrib

Display or change attributes of files or directories.

## Usage
- `attrib` : display all set attributes of files in the current directory
- `attrib path\to\directory` : display all set attributes of files in a specific directory
- `attrib /d` : display all set attributes of files and directories in the current directory
- `attrib /s` : display all set attributes of files in the current directory and sub-directories
- `attrib +r|a|s|h|i path\to\file_or_directory1 path\to\file_or_directory2 ...` : add the `read-only` or `archive` or `system` or `hidden` or `not content indexed` attribute to files or directories
- `attrib -r|a|s|h|i path\to\file_or_directory1 path\to\file_or_directory2 ...` : remove a specific attribute of files or directories

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, desktop, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/attrib
