# llvm-strip-19

Object stripping tool.

## Usage
- `llvm-strip [options] inputs...` : the general form, as the manual gives it
- `llvm-strip-19 --discard-all` : remove most local symbols from the output
- `llvm-strip-19 --enable-deterministic-archives` : enable deterministic mode when stripping archives, i.e
- `llvm-strip-19 --help` : print a summary of command line options
- `llvm-strip-19 --no-strip-all` : disable --strip-all

## Links
- Documentation: https://github.com/llvm/llvm-project/labels/tools:llvm-objcopy%2Fstrip
