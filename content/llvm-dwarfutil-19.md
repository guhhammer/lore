# llvm-dwarfutil-19

A tool to copy and manipulate debug info.

## Usage
- `llvm-dwarfutil [options] input output` : the general form, as the manual gives it
- `llvm-dwarfutil-19 --garbage-collection` : removes  pieces  of debug information related to discarded sections
- `llvm-dwarfutil-19 --help` : print a summary of command line options
- `llvm-dwarfutil-19 --no-garbage-collection` : disable --garbage-collection
- `llvm-dwarfutil-19 --no-odr-deduplication` : disable --odr-deduplication

## Categories
development, files

## Links
- Documentation: https://github.com/llvm/llvm-project/labels/tools:llvm-dwarfutil/
