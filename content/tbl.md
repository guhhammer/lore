# tbl

Table preprocessor for the groff (GNU Troff) document formatting system.

## Usage
- `tbl path/to/input_file >path/to/output.roff` : process input with tables, saving the output for future typesetting with groff to PostScript
- `tbl -T pdf path/to/input.tbl | groff -me -T pdf >path/to/output.pdf` : typeset input with tables to PDF using the [me] macro package

## See also
- groff
- troff

## Categories
documents, files, media

## Links
- More information: https://manned.org/tbl
- tldr page: https://tldr.inbrowser.app/pages/common/tbl
