# shfmt

Shell parser, formatter, and interpreter.

## Usage
- `shfmt path/to/file` : print a formatted version of a shell script
- `shfmt -l path/to/directory` : list unformatted files
- `shfmt -w path/to/file` : write the result to the file instead of printing it to the terminal
- `shfmt -s path/to/file` : simplify the code, removing redundant pieces of syntax (e.g. removing "$" from vars in expressions)
- `shfmt -i 4 path/to/file` : specify the amount of spaces to use for indentation (0 for tabs, which is also the default)
- `shfmt -i 2 -ci -w path/to/file` : format the code according to Google's style guide

## Categories
files, shell, development

## Links
- More information: https://pkg.go.dev/mvdan.cc/sh#section-readme
- tldr page: https://tldr.inbrowser.app/pages/common/shfmt
