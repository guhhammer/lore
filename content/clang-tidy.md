# clang-tidy

An LLVM-based C/C++ linter to find style violations, bugs, and security flaws through static analysis.

## Usage
- `clang-tidy path/to/file.cpp` : run default checks on a source file
- `clang-tidy path/to/file.cpp -checks=-*,cppcoreguidelines-*` : don't run any checks other than the `cppcoreguidelines` checks on a file
- `clang-tidy -checks=* -list-checks` : list all available checks
- `clang-tidy path/to/file.cpp -- -Ipath/to/include -Ddefinitions` : specify defines and includes as compilation options (after `--`)

## Categories
files

## Links
- More information: https://clang.llvm.org/extra/clang-tidy/
- tldr page: https://tldr.inbrowser.app/pages/common/clang-tidy
