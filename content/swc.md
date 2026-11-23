# swc

JavaScript and TypeScript compiler written in Rust.

## Usage
- `swc path/to/file` : transpile a specified input file and output to `stdout`
- `swc path/to/file -w` : transpile the input file every time it is changed
- `swc path/to/input_file -o path/to/output_file` : transpile a specified input file and output to a specific file
- `swc path/to/input_directory -d path/to/output_directory` : transpile a specified input directory and output to a specific directory
- `swc path/to/input_directory --config-file path/to/.swcrc` : transpile a specified input directory using a specific configuration file
- `swc path/to/input_directory --ignore path/to/ignored_file1 path/to/ignored_file2 ...` : ignore files in a directory specified using `glob` path

## Categories
files, development

## Links
- More information: https://swc.rs/docs/usage/cli
- tldr page: https://tldr.inbrowser.app/pages/common/swc
