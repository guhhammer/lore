# llvm-dis

Convert LLVM bitcode files into human-readable LLVM Intermediate Representation (IR).

## Usage
- `llvm-dis path/to/input.bc -o -` : convert a bitcode file as LLVM IR and write the result to `stdout`
- `llvm-dis path/to/file.bc` : convert a bitcode file to an LLVM IR file with the same filename
- `llvm-dis path/to/input.bc -o path/to/output.ll` : convert a bitcode file to LLVM IR, writing the result to the specified file

## Categories
files

## Links
- More information: https://www.llvm.org/docs/CommandGuide/llvm-dis.html
- tldr page: https://tldr.inbrowser.app/pages/common/llvm-dis
