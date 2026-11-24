# clang

Compile C, C++, and Objective-C source files. Can be used as a drop-in replacement for GCC. Part of LLVM.

## Usage
- `clang path/to/source1.c path/to/source2.c ... -o path/to/output_executable` : compile multiple source files into an executable
- `clang path/to/source.c -Wall -o output_executable` : activate output of all errors and warnings
- `clang path/to/source.c -Wall -g -Og -o path/to/output_executable` : show common warnings, debug symbols in output, and optimize without affecting debugging
- `clang path/to/source.c -o path/to/output_executable -Ipath/to/header -Lpath/to/library -llibrary_name` : include libraries from a different path
- `clang -S -emit-llvm path/to/source.c -o path/to/output.ll` : compile source code into LLVM Intermediate Representation (IR)
- `clang -c path/to/source.c` : compile source code into an object file without linking
- `clang path/to/source.c -O1|2|3|fast -o path/to/output_executable` : optimize the compiled program for performance
- `clang --version` : display version

## Categories
files, development

## Links
- More information: https://clang.llvm.org/docs/ClangCommandLineReference.html
- tldr page: https://tldr.inbrowser.app/pages/common/clang
- Wikipedia: https://en.wikipedia.org/wiki/Clang
