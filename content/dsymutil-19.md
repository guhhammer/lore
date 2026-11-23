# dsymutil-19

Manipulate archived DWARF debug symbol files.

## Usage
- `dsymutil [options] executable` : the general form, as the manual gives it
- `dsymutil-19 --arch <arch>` : link DWARF debug information only for specified CPU architecture types
- `dsymutil-19 --fat64` : use a 64-bit header when emitting universal binaries
- `dsymutil-19 --flat` : produce a flat dSYM file
- `dsymutil-19 --gen-reproducer` : generate a reproducer consisting of the input object files

## Categories
development

## Links
- Documentation: https://llvm.org/
