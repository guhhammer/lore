# llvm-libtool-darwin-19

LLVM tool for creating libraries for Darwin.

## Usage
- `llvm-libtool-darwin [options] <input files>` : the general form, as the manual gives it
- `llvm-libtool-darwin-19 -D` : use zero for timestamps and UIDs/GIDs
- `llvm-libtool-darwin-19 -h` : show help and usage for this command
- `llvm-libtool-darwin-19 -no_warning_for_no_symbols` : do not warn about files that have no symbols
- `llvm-libtool-darwin-19 -warnings_as_errors` : produce a non-zero exit status if any warnings are emitted

## Links
- Documentation: https://llvm.org/
