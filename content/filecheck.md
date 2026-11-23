# filecheck

Flexible pattern matching file verifier. It is typically used from LLVM regression tests and forms a part of a `lit` test.

## Usage
- `FileCheck --input-file=path/to/input_file path/to/check_file` : match `input_file` content with pattern file `check_file`
- `echo "some_text" | FileCheck path/to/check_file` : match input from the `stdin` with pattern file `check_file`
- `echo "some_text" | FileCheck --check-prefix=prefix path/to/check_file` : match with the specified custom check `prefix` (Note: The default prefix is `CHECK`)
- `echo "some_text" | FileCheck -v path/to/check_file` : print good directive pattern matches
- `llvm-as path/to/llvm_code.ll | FileCheck path/to/check_file` : input `llvm_code.ll` into llvm-as, then pipe the output into FileCheck to match

## Categories
files, development, text

## Links
- More information: https://llvm.org/docs/CommandGuide/FileCheck.html
- tldr page: https://tldr.inbrowser.app/pages/common/filecheck
