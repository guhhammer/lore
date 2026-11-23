# grex

Generate `regex`s.

## Usage
- `grex string1 string2 ...` : generate a simple `regex`
- `grex -i string1 string2 ...` : generate a case-insensitive `regex`
- `grex -d string1 string2 ...` : replace digits with `\d`
- `grex -w string1 string2 ...` : replace Unicode word character with `\w`
- `grex -s string1 string2 ...` : replace spaces with `\s`
- `grex -r string1 string2 ...` : detect repeating patterns in the input and shorten them using `{min,max}` quantifiers
- `grex -f path/to/file` : generate `regex` of test cases (separated by newline) from a file
- `grex --no-anchors -g string1 string2 ...` : do not generate anchors and non-capture groups

## Categories
text, files

## Links
- More information: https://github.com/pemistahl/grex#5-how-to-use
- tldr page: https://tldr.inbrowser.app/pages/common/grex
