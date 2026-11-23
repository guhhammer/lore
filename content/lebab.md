# lebab

A JavaScript modernizer for transpiling code to ES6/ES7. Transformations must be provided for all examples.

## Usage
- `lebab --transform transformation1,transformation2,...` : transpile using one or more comma-separated transformations
- `lebab path/to/input_file` : transpile a file to `stdout`
- `lebab path/to/input_file --out-file path/to/output_file` : transpile a file to the specified output file
- `lebab --replace directory|glob|file` : replace all `.js` files in-place in the specified directory, `glob`, or file
- `lebab --help` : display help

## Categories
files

## Links
- More information: https://github.com/lebab/lebab
- tldr page: https://tldr.inbrowser.app/pages/common/lebab
