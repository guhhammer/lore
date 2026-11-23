# lli

Directly execute programs from LLVM bitcode.

## Usage
- `lli path/to/file.ll` : execute a bitcode or IR file
- `lli path/to/file.ll argument1 argument2 ...` : execute with command-line arguments
- `lli -O3 path/to/file.ll` : enable all optimizations
- `lli --dlopen=path/to/library.dll path/to/file.ll` : load a dynamic library before linking

## Categories
files

## Links
- More information: https://www.llvm.org/docs/CommandGuide/lli.html
- tldr page: https://tldr.inbrowser.app/pages/common/lli
