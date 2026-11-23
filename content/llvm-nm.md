# llvm-nm

This command is an alias of `nm`.

## Usage
- `nm -g path/to/file.o` : list global (extern) functions in a file (prefixed with T)
- `nm -u path/to/file.o` : list only undefined symbols in a file
- `nm -a path/to/file.o` : list all symbols, even debugging symbols
- `nm -C path/to/file.o` : demangle C++ symbols (make them readable)

## Categories
files, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/llvm-nm
