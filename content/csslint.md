# csslint

Lint CSS code.

## Usage
- `csslint file.css` : lint a single CSS file
- `csslint file1.css file2.css ...` : lint multiple CSS files
- `csslint --list-rules` : list all possible style rules
- `csslint --errors=errors,universal-selector,imports file.css` : treat certain rules as errors (which results in a non-zero exit code)
- `csslint --warnings=box-sizing,selector-max,floats file.css` : treat certain rules as warnings
- `csslint --ignore=ids,rules-count,shorthand file.css` : ignore specific rules

## Categories
files, network

## Links
- More information: https://github.com/CSSLint/csslint/wiki/Command-line-interface
- tldr page: https://tldr.inbrowser.app/pages/common/csslint
