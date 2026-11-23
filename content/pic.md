# pic

Picture preprocessor for the groff (GNU Troff) document formatting system.

## Usage
- `pic path/to/input.pic >path/to/output.roff` : process input with pictures, saving the output for future typesetting with groff to PostScript
- `pic -T pdf path/to/input.pic | groff -me -T pdf >path/to/output.pdf` : typeset input with pictures to PDF using the [me] macro package

## See also
- groff
- troff

## Categories
documents, media, files

## Links
- More information: https://manned.org/pic
- tldr page: https://tldr.inbrowser.app/pages/common/pic
