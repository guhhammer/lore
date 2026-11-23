# hd

This command is an alias of `hexdump`.

## Usage
- `hexdump path/to/file` : print the hexadecimal representation of a file, replacing duplicate lines by `*`
- `hexdump -C path/to/file` : display the input offset in hexadecimal and its ASCII representation in two columns
- `hexdump -C -n number_of_bytes path/to/file` : display the hexadecimal representation of a file, but interpret only a specific number of bytes of the input
- `hexdump -v path/to/file` : do not suppress duplicate lines (verbose mode)
- `hexdump -e 'element_format .. end_format' path/to/file` : format output using printf-like format string

## Categories
files, shell, text

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/hd
