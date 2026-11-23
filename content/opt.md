# opt

Run optimizations and analyze LLVM source files.

## Usage
- `opt -passname path/to/file.bc -S -o file_opt.bc` : run an optimization or analysis on a bitcode file
- `opt -dot-cfg -S path/to/file.bc -disable-output` : output the Control Flow Graph of a function to a `.dot` file
- `opt -O2 path/to/file.bc -S -o path/to/output_file.bc` : optimize the program at level 2 and output the result to another file

## Categories
files

## Links
- More information: https://llvm.org/docs/CommandGuide/opt.html
- tldr page: https://tldr.inbrowser.app/pages/common/opt
