# llvm-install-name-tool-19

LLVM tool for manipulating install-names and rpaths.

## Usage
- `llvm-install-name-tool [options] input` : the general form, as the manual gives it
- `llvm-install-name-tool-19 -add_rpath <rpath>` : add an rpath named <rpath> to the specified binary
- `llvm-install-name-tool-19 -delete_rpath <rpath>` : delete an rpath named <rpath> from the specified binary
- `llvm-install-name-tool-19 -delete_all_rpaths` : deletes all rpaths from the binary
- `llvm-install-name-tool-19 --help` : print a summary of command line options

## Categories
packages

## Links
- Documentation: https://github.com/llvm/llvm-project/labels/tools:llvm-objcopy/strip/
