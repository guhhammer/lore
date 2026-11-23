# llvm-strip

Object stripping tool.

## Usage
- `llvm-strip [options] inputs...` : the general form, as the manual gives it
- `llvm-strip --discard-all` : remove most local symbols from the output
- `llvm-strip --enable-deterministic-archives` : enable deterministic mode when stripping archives, i.e
- `llvm-strip --help` : print a summary of command line options
- `llvm-strip --no-strip-all` : disable --strip-all

## Links
- Documentation: https://github.com/llvm/llvm-project/labels/tools:llvm-objcopy%2Fstrip
