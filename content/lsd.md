# lsd

List directory contents. The next generation `ls` command, written in Rust.

## Usage
- `lsd -1` : list files and directories, one per line
- `lsd -a` : list all files and directories, including hidden ones, in the current directory
- `lsd -F` : list files and directories with trailing `/` added to directory names
- `lsd -lha` : list all files and directories in long format (permissions, ownership, size in human-readable format, and modification date)
- `lsd -lS` : list files and directories in long format, sorted by size (descending)
- `lsd -ltr` : list files and directories in long format, sorted by modification date (oldest first)
- `lsd -d */` : only list directories
- `lsd --tree -d` : recursively list all directories in a tree format

## Categories
files

## Links
- More information: https://github.com/lsd-rs/lsd/blob/main/doc/lsd.md
- tldr page: https://tldr.inbrowser.app/pages/common/lsd
