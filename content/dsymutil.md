# dsymutil

Manipulate archived DWARF debug symbol files.

## Usage
- `dsymutil [options] executable` : the general form, as the manual gives it
- `dsymutil --arch <arch>` : link DWARF debug information only for specified CPU architecture types
- `dsymutil --fat64` : use a 64-bit header when emitting universal binaries
- `dsymutil --flat` : produce a flat dSYM file
- `dsymutil --gen-reproducer` : generate a reproducer consisting of the input object files

## Categories
development

## Links
- Documentation: https://llvm.org/
