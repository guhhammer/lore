# llvm-symbolizer

Convert addresses into source code locations.

## Usage
- `llvm-symbolizer [options] [addresses...]` : the general form, as the manual gives it
- `llvm-symbolizer --color [=<always|auto|never>]` : specify whether to use color in --filter-markup mode
- `llvm-symbolizer --demangle` : print demangled function names, if the names are mangled (e.g
- `llvm-symbolizer --help` : show help and usage for this command
- `llvm-symbolizer --no-inlines` : don't print inlined frames

## Categories
development, documents

## Links
- Documentation: https://llvm.org/
