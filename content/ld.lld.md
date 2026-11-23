# ld.lld

ELF linker from the LLVM project.

## Usage
- `ld.lld [options] objfile ...` : the general form, as the manual gives it
- `ld.lld --allow-multiple-definition` : do not error if a symbol is defined multiple times
- `ld.lld --allow-shlib-undefined` : allow unresolved references in shared libraries
- `ld.lld --apply-dynamic-relocs` : apply link-time values for dynamic relocations
- `ld.lld --as-needed` : only set DT_NEEDED for shared libraries if used

## Categories
development
