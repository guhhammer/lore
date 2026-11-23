# llvm-as

LLVM Intermediate Representation (`.ll`) to Bitcode (`.bc`) assembler.

## Usage
- `llvm-as -o path/to/out.bc path/to/source.ll` : assemble an IR file
- `llvm-as --module-hash -o path/to/out.bc path/to/source.ll` : assemble an IR file and include a module hash in the produced Bitcode file
- `cat path/to/source.ll | llvm-as -o path/to/out.bc` : read an IR file from `stdin` and assemble it

## Categories
files

## Links
- More information: https://llvm.org/docs/CommandGuide/llvm-as.html
- tldr page: https://tldr.inbrowser.app/pages/common/llvm-as
