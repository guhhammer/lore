# llvm-extract

Extract a function from an LLVM module.

## Usage
- `llvm-extract [options] --func function-name [filename]` : the general form, as the manual gives it
- `llvm-extract --alias alias-name` : extract the alias named function-name from the LLVM bitcode
- `llvm-extract --ralias alias-regular-expr` : extract  the alias matching alias-regular-expr from the LLVM bitcode
- `llvm-extract --bb basic-block-specifier` : extract basic blocks(s) specified in basic-block-specifier
- `llvm-extract --delete` : delete specified Globals from Module

## Categories
archives, shell, development

## Links
- Documentation: https://llvm.org/
