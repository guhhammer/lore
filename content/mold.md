# mold

A modern linker, and a faster drop-in replacement for GNU `ld`.

## Usage
- `mold --run command arguments` : run a build command such as `make`, `ninja`, or `cargo`, with `mold` used in place of `ld`
- `mold path/to/file1.o path/to/file2.o ... -o path/to/output_executable` : link object files into an executable
- `mold --shared path/to/file1.o path/to/file2.o ... -o path/to/library.so` : link object files into a shared library
- `mold path/to/file.o -L path/to/library_directory -llibrary_name` : link an object file with a library located in a specific directory
- `mold --print-dependencies path/to/file1.o path/to/file2.o ... -o path/to/output_executable` : print which input files depend on which others for each symbol

## See also
- ld

## Categories
files, development

## Links
- More information: https://github.com/rui314/mold/blob/main/docs/mold.md
- tldr page: https://tldr.inbrowser.app/pages/common/mold
