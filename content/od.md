# od

Display file contents in octal, decimal, or hexadecimal format. Optionally display the byte offsets and/or printable representation for each line.

## Usage
- `od path/to/file` : display file using default settings: octal format, 8 bytes per line, byte offsets in octal, and duplicate lines replaced with `*`
- `od -v path/to/file` : display file in verbose mode, i.e. without replacing duplicate lines with `*`
- `od -t x -A d -v path/to/file` : display file in hexadecimal format (2-byte units), with byte offsets in decimal format
- `od -t x1 -w4 -v path/to/file` : display file in hexadecimal format (1-byte units), and 4 bytes per line
- `od -t xz -A n -v path/to/file` : display file in hexadecimal format along with its character representation, and do not print byte offsets
- `od -N 100 -j 500 -v path/to/file` : read only 100 bytes of a file starting from the 500th byte

## See also
- hexyl
- xxd
- hexdump

## Categories
files, desktop, text

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/od-invocation.html
- tldr page: https://tldr.inbrowser.app/pages/common/od
