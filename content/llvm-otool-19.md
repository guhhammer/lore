# llvm-otool-19

Mach-O dumping tool.

## Usage
- `llvm-otool [option...] [file...]` : the general form, as the manual gives it
- `llvm-otool-19 -arch <value>` : select slice of universal Mach-O file
- `llvm-otool-19 -chained_fixups` : print chained fixup information
- `llvm-otool-19 -C` : print linker optimization hints
- `llvm-otool-19 -dyld_info` : print bind and rebase information

## Categories
documents

## Links
- Documentation: https://github.com/llvm/llvm-project/labels/tools:llvm-objdump/
