# gnujoin

This command is an alias of GNU `join`.

## Usage
- `join path/to/file1 path/to/file2` : join two files on the first (default) field
- `join -t ',' path/to/file1 path/to/file2` : join two files using a comma (instead of a space) as the field separator
- `join -1 3 -2 1 path/to/file1 path/to/file2` : join field3 of file1 with field1 of file2
- `join -a 1 path/to/file1 path/to/file2` : produce a line for each unpairable line for file1
- `cat path/to/file1 | join - path/to/file2` : join a file from `stdin`

## Categories
files, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/gnujoin
