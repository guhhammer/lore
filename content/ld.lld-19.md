# ld.lld-19

ELF linker from the LLVM project.

## Usage
- `ld.lld [options] objfile ...` : the general form, as the manual gives it
- `ld.lld-19 --allow-multiple-definition` : do not error if a symbol is defined multiple times
- `ld.lld-19 --allow-shlib-undefined` : allow unresolved references in shared libraries
- `ld.lld-19 --apply-dynamic-relocs` : apply link-time values for dynamic relocations
- `ld.lld-19 --as-needed` : only set DT_NEEDED for shared libraries if used

## Categories
development
