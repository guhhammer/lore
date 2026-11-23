# rabin2

Get information about binary files (ELF, PE, Java CLASS, Mach-O) - symbols, sections, linked libraries, etc. Comes bundled with `radare2`.

## Usage
- `rabin2 -I path/to/binary` : display general information about a binary (architecture, type, endianness)
- `rabin2 -l path/to/binary` : display linked libraries
- `rabin2 -i path/to/binary` : display symbols imported from libraries
- `rabin2 -z path/to/binary` : display strings contained in the binary
- `rabin2 -j -I path/to/binary` : display the output in JSON

## Categories
desktop, files

## Links
- More information: https://manned.org/rabin2
- tldr page: https://tldr.inbrowser.app/pages/common/rabin2
