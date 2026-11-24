# cmp

Compare two files byte by byte.

## Usage
- `cmp path/to/file1 path/to/file2` : output char and line number of the first difference between two files
- `cmp -b path/to/file1 path/to/file2` : output info of the first difference: char, line number, bytes, and values
- `cmp -l path/to/file1 path/to/file2` : output the byte numbers and values of every difference
- `cmp -s path/to/file1 path/to/file2` : compare files but output nothing, yield only the exit status

## Categories
files

## Links
- More information: https://www.gnu.org/software/diffutils/manual/diffutils.html#Invoking-cmp
- tldr page: https://tldr.inbrowser.app/pages/common/cmp
- Wikipedia: https://en.wikipedia.org/wiki/Cmp_(Unix)
