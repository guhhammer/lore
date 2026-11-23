# llc

Compile LLVM Intermediate Representation or bitcode to target-specific assembly language.

## Usage
- `llc path/to/file.ll` : compile a bitcode or IR file to an assembly file with the same base name
- `llc -O3 path/to/input.ll` : enable all optimizations
- `llc --output path/to/output.s` : output assembly to a specific file
- `llc -relocation-model=pic path/to/input.ll` : emit fully relocatable, position independent code

## Categories
files, development

## Links
- More information: https://www.llvm.org/docs/CommandGuide/llc.html
- tldr page: https://tldr.inbrowser.app/pages/common/llc
