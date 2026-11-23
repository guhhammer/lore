# grap

A charting preprocessor for the groff (GNU Troff) document formatting system.

## Usage
- `grap path/to/input.grap >path/to/output.pic` : process a `grap` file and save the output file for future processing with `pic` and `groff`
- `grap path/to/input.grap | pic -T pdf | groff -me -T pdf >path/to/output.pdf` : typeset a `grap` file to PDF using the [me] macro package, saving the output to a file

## See also
- pic
- groff

## Categories
files, documents, media

## Links
- More information: https://manned.org/grap
- tldr page: https://tldr.inbrowser.app/pages/common/grap
