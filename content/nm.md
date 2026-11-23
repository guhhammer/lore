# nm

List symbol names in object files.

## Usage
- `nm -g path/to/file.o` : list global (extern) functions in a file (prefixed with T)
- `nm -u path/to/file.o` : list only undefined symbols in a file
- `nm -a path/to/file.o` : list all symbols, even debugging symbols
- `nm -C path/to/file.o` : demangle C++ symbols (make them readable)

## Categories
files

## Links
- More information: https://manned.org/nm
- tldr page: https://tldr.inbrowser.app/pages/common/nm
