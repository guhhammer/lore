# clang-check

Check basic errors and work with Clang's Abstract Syntax Tree (AST). Part of Clang's LibTooling and is useful for debugging and analyzing C/C++ code.

## Usage
- `clang-check path/to/file.cpp --` : run default checks on a source file
- `clang-check path/to/file.cpp -ast-dump --` : dump the Abstract Syntax Tree for debugging
- `clang-check path/to/file.cpp -ast-dump -ast-dump-filter FunctionName` : filter AST by Name
- `clang-check path/to/file.cpp -ast-print --` : pretty-Print AST

## Categories
files

## Links
- More information: https://manned.org/clang-check
- tldr page: https://tldr.inbrowser.app/pages/common/clang-check
