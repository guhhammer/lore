# llvm-nm-19

List LLVM bitcode and object file's symbol table.

## Usage
- `llvm-nm [options] [filenames...]` : the general form, as the manual gives it
- `llvm-nm-19 -B` : use BSD output format
- `llvm-nm-19 --debug-syms` : show all symbols, even those usually suppressed
- `llvm-nm-19 --defined-only` : print only symbols defined in this file
- `llvm-nm-19 --demangle` : demangle symbol names

## Categories
files, databases

## Links
- Documentation: https://llvm.org/
