# winedump

Dump the contents of Windows binaries (PE, NE, etc.), demangle C++ symbols, and generate Wine `.spec` files.

## Usage
- `winedump dump path/to/file.dll` : dump the contents of a PE file, DLL, or executable
- `winedump dump -f path/to/file.dll` : dump only the file header information
- `winedump dump -j import|export path/to/file.dll` : dump a specific section, such as the import or export table
- `winedump sym mangled_symbol` : demangle a C++ (MSVC-mangled) symbol
- `winedump spec path/to/file.dll` : generate a `.spec` file and stub implementation code from a DLL
- `winedump -h` : display help

## Categories
files

## Links
- More information: https://manned.org/winedump
- tldr page: https://tldr.inbrowser.app/pages/linux/winedump
