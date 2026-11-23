# gstat

This command is an alias of GNU `stat`.

## Usage
- `stat path/to/file` : display properties about a specific file such as size, permissions, creation date, and access date among others
- `stat -t path/to/file` : display properties about a specific file, only showing the raw result data without labels
- `stat -f path/to/file` : display information about the filesystem where a specific file is located
- `stat -c "%a %n" path/to/file` : show only octal file permissions
- `stat -c "%U %G" path/to/file` : show the owner and group of a specific file
- `stat -c "%s %n" path/to/file` : show the size of a specific file in bytes

## Categories
files, desktop, shell, macos

## Links
- tldr page: https://tldr.inbrowser.app/pages/osx/gstat
