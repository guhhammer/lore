# eqn

Equation preprocessor for the groff (GNU Troff) document formatting system.

## Usage
- `eqn path/to/input.eqn >path/to/output.roff` : process input with equations, saving the output for future typesetting with groff to PostScript
- `eqn -T pdf path/to/input.eqn | groff -me -T pdf >path/to/output.pdf` : typeset an input file with equations to PDF using the [me] macro package

## See also
- troff
- groff

## Categories
documents, files, media

## Links
- More information: https://manned.org/eqn
- tldr page: https://tldr.inbrowser.app/pages/common/eqn
