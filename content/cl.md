# cl

The Microsoft C/C++ compiler for compiling and linking source code files.

## Usage
- `cl path\to\source.c` : compile a source file
- `cl /Fe path\to\output_executable path\to\source.c` : compile and create an executable with a custom name
- `cl /O2 path\to\source.c` : compile a source file with optimization enabled
- `cl /Zi path\to\source.c` : compile a source file and create a debug executable
- `cl path\to\source1.c path\to\source2.c ...` : compile multiple source files
- `cl /Fo path\to\output_directory\ path\to\source.c` : specify the output directory for compiled files
- `cl /WX path\to\source.c` : compile with warnings as errors

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
development, files, windows

## Links
- More information: https://learn.microsoft.com/cpp/build/reference/compiler-command-line-syntax
- tldr page: https://tldr.inbrowser.app/pages/windows/cl
