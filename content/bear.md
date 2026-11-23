# bear

A tool to generate compilation databases for `clang` tooling.

## Usage
- `bear -- make` : generate `compile_commands.json` by running a build command
- `bear --output path/to/compile_commands.json -- make` : generate compilation database with a custom output file name
- `bear --append -- make` : append results to an existing `compile_commands.json` file
- `bear --verbose -- make` : run in verbose mode to get detailed output
- `bear --force-preload -- make` : force `bear` to use the preload method for command interception

## Categories
files, text

## Links
- More information: https://github.com/rizsotto/Bear
- tldr page: https://tldr.inbrowser.app/pages/common/bear
