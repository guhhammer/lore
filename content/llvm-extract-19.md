# llvm-extract-19

Extract a function from an LLVM module.

## Usage
- `llvm-extract [options] --func function-name [filename]` : the general form, as the manual gives it
- `llvm-extract-19 --alias alias-name` : extract the alias named function-name from the LLVM bitcode
- `llvm-extract-19 --ralias alias-regular-expr` : extract  the alias matching alias-regular-expr from the LLVM bitcode
- `llvm-extract-19 --bb basic-block-specifier` : extract basic blocks(s) specified in basic-block-specifier
- `llvm-extract-19 --delete` : delete specified Globals from Module

## Categories
archives, shell, development

## Links
- Documentation: https://llvm.org/
