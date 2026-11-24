# readelf

Display information about ELF files.

## Usage
- `readelf -a path/to/binary` : display all information about the ELF file
- `readelf -e path/to/binary` : display all the headers present in the ELF file
- `readelf -s path/to/binary` : display the entries in symbol table section of the ELF file, if it has one
- `readelf -h path/to/binary` : display ELF header information
- `readelf -S path/to/binary` : display ELF section header information
- `readelf -w path/to/binary` : display DWARF information

## Categories
desktop, files

## Links
- More information: https://manned.org/readelf
- tldr page: https://tldr.inbrowser.app/pages/linux/readelf
- Wikipedia: https://en.wikipedia.org/wiki/Readelf
