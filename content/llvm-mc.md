# llvm-mc

LLVM Machine Code Playground. It provides a set of tools for working with LLVM machine code. Part of LLVM.

## Usage
- `llvm-mc --filetype=obj -o path/to/output.o path/to/input.s` : assemble assembly code file into object file with machine code
- `llvm-mc --disassemble -o path/to/output.s path/to/input.o` : disassemble object file with machine code into assembly code file
- `llvm-mc -o path/to/output.s path/to/input.bc` : compile LLVM bit code file into assembly code
- `echo "addl %eax, %ebx" | llvm-mc -show-encoding -show-inst` : assemble assembly code from `stdin` and show encoding to `stdout`
- `echo "0xCD 0x21" | llvm-mc --disassemble -triple=target_name` : disassemble machine code from `stdin` for specified triple

## Categories
files

## Links
- More information: https://llvm.org/docs/CommandGuide/llvm-mc.html
- tldr page: https://tldr.inbrowser.app/pages/common/llvm-mc
