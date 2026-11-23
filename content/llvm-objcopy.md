# llvm-objcopy

Object copying and editing tool.

## Usage
- `llvm-objcopy [options] input [output]` : the general form, as the manual gives it
- `llvm-objcopy --add-gnu-debuglink <debug-file>` : add a .gnu_debuglink section for <debug-file> to the output
- `llvm-objcopy --binary-architecture <arch>` : ignored for compatibility
- `llvm-objcopy --discard-all` : remove  most local symbols from the output
- `llvm-objcopy --enable-deterministic-archives` : enable deterministic mode when copying archives, i.e

## Categories
development

## Links
- Documentation: https://github.com/llvm/llvm-project/labels/tools:llvm-objcopy/strip/
