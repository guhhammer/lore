# clang-format

Auto-format C/C++/Java/JavaScript/Objective-C/Protobuf/C# code.

## Usage
- `clang-format path/to/file` : format a file and print the result to `stdout`
- `clang-format -i path/to/file` : format a file in-place
- `clang-format --style LLVM|GNU|Google|Chromium|Microsoft|Mozilla|WebKit path/to/file` : format a file using a predefined coding style
- `clang-format --style file path/to/file` : format a file using the `.clang-format` file in one of the parent directories of the source file
- `clang-format --style LLVM|GNU|Google|Chromium|Microsoft|Mozilla|WebKit --dump-config >.clang-format` : generate a custom `.clang-format` file

## Categories
files

## Links
- More information: https://clang.llvm.org/docs/ClangFormat.html
- tldr page: https://tldr.inbrowser.app/pages/common/clang-format
