# getopt

Parse command-line arguments.

## Usage
- `getopt -o vV -l verbose,version -- --version --verbose` : parse optional `verbose`/`version` flags with shorthands
- `getopt -o f: -l file: -- --file=somefile` : add a `--file` option with a required argument with shorthand `-f`
- `getopt -o v:: -l verbose:: -- --verbose arg` : add a `--verbose` option with an optional argument with shorthand `-v`, and pass a non-option parameter `arg`
- `getopt -o rv::s::t: -l verbose,source::,target: -- -v --target target` : accept a `-r` and `--verbose` flag, a `--accept` option with an optional argument, and add a `--target` with a required argument option with shorthands

## Categories
files, text

## Links
- More information: https://manned.org/getopt
- tldr page: https://tldr.inbrowser.app/pages/linux/getopt
