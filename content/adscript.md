# adscript

Compiler for Adscript files.

## Usage
- `adscript --output path/to/file.o path/to/input_file.adscript` : compile a file to an object file
- `adscript --executable --output path/to/file path/to/input_file.adscript` : compile and link a file to a standalone executable
- `adscript --llvm-ir --output path/to/file.ll path/to/input_file.adscript` : compile a file to LLVM IR instead of native machine code
- `adscript --target-triple i386-linux-elf --output path/to/file.o path/to/input_file.adscript` : cross-compile a file to an object file for a foreign CPU architecture or operating system

## Categories
files, development

## Links
- More information: https://github.com/Amplus2/Adscript
- tldr page: https://tldr.inbrowser.app/pages/common/adscript
