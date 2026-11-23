# nasm

The Netwide Assembler, a portable 80x86 assembler.

## Usage
- `nasm source.asm` : assemble `source.asm` into a binary file `source`, in the (default) raw binary format
- `nasm -f format source.asm -o output_file` : assemble `source.asm` into a binary file `output_file`, in the specified format
- `nasm -hf` : list valid output formats (along with basic nasm help)
- `nasm -l list_file source.asm` : assemble and generate an assembly listing file
- `nasm -i path/to/include_directory/ source.asm` : add a directory (must be written with trailing slash) to the include file search path before assembling

## Categories
files

## Links
- More information: https://www.nasm.us/doc/nasm02.html
- tldr page: https://tldr.inbrowser.app/pages/common/nasm
