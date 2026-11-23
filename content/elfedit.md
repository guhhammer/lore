# elfedit

Update the ELF header and program properties of ELF files.

## Usage
- `elfedit --output-mach x86-64|i386|iamcu|... path/to/file` : change the machine type of an ELF file
- `elfedit --input-mach i386 --output-mach iamcu path/to/file1 path/to/file2 ...` : change the machine type only in files that currently have a specific machine type
- `elfedit --output-type rel|exec|dyn path/to/file` : change the file type to relocatable, executable, or shared object
- `elfedit --output-osabi none|Linux|FreeBSD|... path/to/file` : change the OSABI
- `elfedit --output-abiversion 0..255 path/to/file` : change the ABI version
- `elfedit --enable-x86-feature ibt|shstk|lam_u48|lam_u57 path/to/file` : enable an x86 feature in an executable or shared library
- `elfedit --disable-x86-feature ibt|shstk|lam_u48|lam_u57 path/to/file` : disable an x86 feature in an executable or shared library
- `elfedit -h` : display help

## See also
- readelf
- objcopy

## Categories
files

## Links
- More information: https://manned.org/elfedit
- tldr page: https://tldr.inbrowser.app/pages/linux/elfedit
