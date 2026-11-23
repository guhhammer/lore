# musl-gcc

A wrapper around `gcc` that automatically sets options for linking against musl libc. All options specified are passed directly to `gcc`.

## Usage
- `gcc path/to/source1.c path/to/source2.c ... -o path/to/output_executable` : compile multiple source files into an executable
- `gcc path/to/source.c -Wall -o output_executable` : activate output of all errors and warnings
- `gcc path/to/source.c -Wall -g -Og -o path/to/output_executable` : show common warnings, debug symbols in output, and optimize without affecting debugging
- `gcc path/to/source.c -o path/to/output_executable -Ipath/to/header -Lpath/to/library -llibrary_name` : include libraries from a different path
- `gcc -S path/to/source.c` : compile source code into Assembler instructions
- `gcc -c path/to/source.c` : compile source code into an object file without linking
- `gcc path/to/source.c -O1|2|3|fast -o path/to/output_executable` : optimize the compiled program for performance
- `gcc --version` : display version

## Categories
files, development

## Links
- More information: https://manned.org/musl-gcc
- tldr page: https://tldr.inbrowser.app/pages/common/musl-gcc
