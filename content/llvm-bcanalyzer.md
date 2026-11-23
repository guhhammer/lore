# llvm-bcanalyzer

LLVM Bitcode (`.bc`) analyzer.

## Usage
- `llvm-bcanalyzer path/to/file.bc` : print statistics about a Bitcode file
- `llvm-bcanalyzer -dump path/to/file.bc` : print an SGML representation and statistics about a Bitcode file
- `cat path/to/file.bc | llvm-bcanalyzer` : read a Bitcode file from `stdin` and analyze it

## Categories
files

## Links
- More information: https://llvm.org/docs/CommandGuide/llvm-bcanalyzer.html
- tldr page: https://tldr.inbrowser.app/pages/common/llvm-bcanalyzer
