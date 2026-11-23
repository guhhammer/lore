# llvm-g++

This command is an alias of `clang++`.

## Usage
- `clang++ path/to/source1.cpp path/to/source2.cpp ... -o path/to/output_executable` : compile a set of source code files into an executable binary
- `clang++ path/to/source.cpp -Wall -o output_executable` : activate output of all errors and warnings
- `clang++ path/to/source.cpp -Wall -g -Og -o path/to/output_executable` : show common warnings, debug symbols in output, and optimize without affecting debugging
- `clang++ path/to/source.cpp -std=c++20 -o path/to/output_executable` : choose a language standard to compile for
- `clang++ path/to/source.cpp -o path/to/output_executable -Ipath/to/headers -Lpath/to/library -lpath/to/library_name` : include libraries located at a different path than the source file
- `clang++ -S -emit-llvm path/to/source.cpp -o path/to/output.ll` : compile source code into LLVM Intermediate Representation (IR)
- `clang++ path/to/source.cpp -O1|2|3|fast -o path/to/output_executable` : optimize the compiled program for performance
- `clang++ --version` : display version

## Categories
files, development, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/llvm-g++
