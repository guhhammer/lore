# babel

A transpiler which converts code from JavaScript ES6/ES7 syntax to ES5 syntax.

## Usage
- `babel path/to/file` : transpile a specified input file and output to `stdout`
- `babel path/to/input_file --out-file path/to/output_file` : transpile a specified input file and output to a specific file
- `babel path/to/input_file --watch` : transpile the input file every time it is changed
- `babel path/to/input_directory` : transpile a whole directory of files
- `babel path/to/input_directory --ignore ignored_file1,ignored_file2,...` : ignore specified comma-separated files in a directory
- `babel path/to/input_file --minified` : transpile and output as minified JavaScript
- `babel path/to/input_file --presets preset1,preset2,...` : choose a set of presets for output formatting
- `babel --help` : display help

## Categories
files

## Links
- More information: https://babeljs.io/docs/babel-cli
- tldr page: https://tldr.inbrowser.app/pages/common/babel
