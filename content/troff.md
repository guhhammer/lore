# troff

Typesetting processor for the groff (GNU Troff) document formatting system.

## Usage
- `troff path/to/input.roff | grops >path/to/output.ps` : format output for a PostScript printer, saving the output to a file
- `troff -me path/to/input.roff | grops >path/to/output.ps` : format output for a PostScript printer using the me macro package, saving the output to a file
- `troff -T ascii -man path/to/input.roff | grotty` : format output as ASCII text using the man macro package
- `troff -T pdf path/to/input.roff | gropdf >path/to/output.pdf` : format output as a pdf file, saving the output to a file

## See also
- groff

## Categories
files, documents, media

## Links
- More information: https://manned.org/troff
- tldr page: https://tldr.inbrowser.app/pages/common/troff
- Wikipedia: https://en.wikipedia.org/wiki/Troff
