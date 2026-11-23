# zek

Generate a Go struct from XML.

## Usage
- `cat path/to/input.xml | zek` : generate a Go struct from a given XML from `stdin` and display output on `stdout`
- `curl -s https://url/to/xml | zek -o path/to/output.go` : generate a Go struct from a given XML from `stdin` and send output to a file
- `cat path/to/input.xml | zek -p -o path/to/output.go` : generate an example Go program from a given XML from `stdin` and send output to a file

## Categories
text, files, network

## Links
- More information: https://github.com/miku/zek#usage
- tldr page: https://tldr.inbrowser.app/pages/common/zek
