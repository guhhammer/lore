# as

Portable GNU assembler. Primarily intended to assemble output from `gcc` to be used by `ld`.

## Usage
- `as path/to/file.s` : assemble a file, writing the output to `a.out`
- `as path/to/file.s -o path/to/output_file.o` : assemble the output to a given file
- `as -f path/to/file.s` : generate output faster by skipping whitespace and comment preprocessing. (Should only be used for trusted compilers)
- `as -I path/to/directory path/to/file.s` : include a given path to the list of directories to search for files specified in `.include` directives

## Categories
files

## Links
- More information: https://manned.org/as
- tldr page: https://tldr.inbrowser.app/pages/linux/as
