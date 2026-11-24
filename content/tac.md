# tac

Display and concatenate files with lines in reversed order.

## Usage
- `tac path/to/file1 path/to/file2 ...` : concatenate specific files in reversed order
- `cat path/to/file | tac` : display `stdin` in reversed order
- `tac -s separator path/to/file1 path/to/file2 ...` : use a specific separator
- `tac -r -s separator path/to/file1 path/to/file2 ...` : use a specific `regex` as a separator
- `tac -b path/to/file1 path/to/file2 ...` : use a separator before each file

## See also
- cat

## Categories
files, desktop, text

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/tac-invocation.html
- tldr page: https://tldr.inbrowser.app/pages/common/tac
- Wikipedia: https://en.wikipedia.org/wiki/Cat_(Unix)
