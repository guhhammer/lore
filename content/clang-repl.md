# clang-repl

Interactive REPL (Read-Evaluate-Print-Loop) environment for C++.

## Usage
- `clang-repl` : run the C++ REPL
- `clang-repl '#include "test.cpp"'` : run C++ from a file
- `clang-repl '#include <iostream>' 'std::cout << "test\n";' '...'` : run C++ code directly from the command line
- `24|"text"` : [Interactive] Evaluate a value
- `%undo` : [Interactive] Undo the last statement
- `%lib library.so.1` : [Interactive] Link a dynamic library
- `%quit|<Ctrl c>` : [Interactive] Exit the environment
- `%help` : [Interactive] Display help

## Categories
development, documents

## Links
- More information: https://clang.llvm.org/docs/ClangRepl.html#clang-repl-usage
- tldr page: https://tldr.inbrowser.app/pages/common/clang-repl
