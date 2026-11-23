# hexdump

Display file contents in hexadecimal, decimal, octal, or ASCII. Useful for inspecting dump file, binary data, or debug output.

## Usage
- `hexdump path/to/file` : print the hexadecimal representation of a file, replacing duplicate lines by `*`
- `hexdump -C path/to/file` : display the input offset in hexadecimal and its ASCII representation in two columns
- `hexdump -C -n number_of_bytes path/to/file` : display the hexadecimal representation of a file, but interpret only a specific number of bytes of the input
- `hexdump -v path/to/file` : do not suppress duplicate lines (verbose mode)
- `hexdump -e 'element_format .. end_format' path/to/file` : format output using printf-like format string

## See also
- hexyl
- od
- xxd

## Categories
files, desktop, development

## Links
- More information: https://manned.org/man/freebsd/hexdump.1
- tldr page: https://tldr.inbrowser.app/pages/common/hexdump
