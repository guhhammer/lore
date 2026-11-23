# llvm-pdbutil

PDB File forensics and diagnostics.

## Usage
- `llvm-pdbutil [subcommand] [options]` : the general form, as the manual gives it
- `llvm-pdbutil -no-compiler-generated` : don't show compiler generated types and symbols
- `llvm-pdbutil -no-enum-definitions` : when dumping an enum, don't show the full enum (e.g
- `llvm-pdbutil -no-system-libs` : don't show symbols from system libraries
- `llvm-pdbutil -all` : implies all other options in this category

## Categories
files

## Links
- Documentation: https://llvm.org/
