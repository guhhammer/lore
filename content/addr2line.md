# addr2line

Convert addresses of a binary into file names and line numbers.

## Usage
- `addr2line -e path/to/executable address` : display the filename and line number of the source code from an instruction address of an executable
- `addr2line -e path/to/executable -f address` : display the function name, filename, and line number
- `addr2line -e path/to/executable -f -C address` : demangle the function name for C++ code

## Categories
files, text

## Links
- More information: https://manned.org/addr2line
- tldr page: https://tldr.inbrowser.app/pages/linux/addr2line
