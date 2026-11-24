# join

Join lines of two sorted files on a common field.

## Usage
- `join path/to/file1 path/to/file2` : join two files on the first (default) field
- `join -t ',' path/to/file1 path/to/file2` : join two files using a comma (instead of a space) as the field separator
- `join -1 3 -2 1 path/to/file1 path/to/file2` : join field3 of file1 with field1 of file2
- `join -a 1 path/to/file1 path/to/file2` : produce a line for each unpairable line for file1
- `cat path/to/file1 | join - path/to/file2` : join a file from `stdin`

## Categories
files, text

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/join-invocation.html
- tldr page: https://tldr.inbrowser.app/pages/common/join
- Wikipedia: https://en.wikipedia.org/wiki/Join_(Unix)
