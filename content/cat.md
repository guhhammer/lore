# cat

Print and concatenate files.

## Usage
- `cat path/to/file` : print the contents of a file to `stdout`
- `cat path/to/file1 path/to/file2 ... >path/to/output_file` : concatenate several files into an output file
- `cat path/to/file1 path/to/file2 ... >>path/to/output_file` : append several files to an output file
- `cat -u /dev/tty12 >/dev/tty13` : copy the contents of a file into an output file without buffering
- `cat - >path/to/file` : write `stdin` to a file

## See also
- bat

## Categories
text, files

## Links
- More information: https://manned.org/cat.1posix
- tldr page: https://tldr.inbrowser.app/pages/common/cat
- Wikipedia: https://en.wikipedia.org/wiki/Cat_(Unix)
