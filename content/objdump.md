# objdump

View information about object files.

## Usage
- `objdump -f path/to/binary` : display the file header information
- `objdump -x path/to/binary` : display all header information
- `objdump -d path/to/binary` : display the disassembled output of executable sections
- `objdump -d path/to/binary -M intel` : display the disassembled executable sections in Intel syntax
- `objdump -d path/to/binary --visualize-jumps=color|extended-color --disassembler-color=color|extended-color` : display the disassembled executable sections with jump visualizations and syntax highlighting
- `objdump -t path/to/binary` : display the symbol table
- `objdump -s path/to/binary` : display a complete binary hex dump of all sections

## Categories
files, desktop, media

## Links
- More information: https://manned.org/objdump
- tldr page: https://tldr.inbrowser.app/pages/common/objdump
- Wikipedia: https://en.wikipedia.org/wiki/Objdump
