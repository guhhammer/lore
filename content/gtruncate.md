# gtruncate

This command is an alias of GNU `truncate`.

## Usage
- `truncate -s 10G path/to/file` : set a size of 10 GB to an existing file, or create a new file with the specified size
- `truncate -s +50M path/to/file` : extend the file size by 50 MiB, fill with holes (which reads as zero bytes)
- `truncate -s -2G path/to/file` : shrink the file by 2 GiB, by removing data from the end of file
- `truncate -s 0 path/to/file` : empty the file's content
- `truncate -s 0 -c path/to/file` : empty the file's content, but do not create the file if it does not exist

## Categories
files, shell, macos

## Links
- tldr page: https://tldr.inbrowser.app/pages/osx/gtruncate
