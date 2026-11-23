# latexpand

Simplify LaTeX source files by removing comments and resolving `\include`s, `\input`s, etc.

## Usage
- `latexpand -o path/to/output.tex path/to/file.tex` : simplify the specified source file and save the result to the specified output file
- `latexpand --keep-comments -o path/to/output.tex path/to/file.tex` : do not remove comments
- `latexpand --keep-includes -o path/to/output.tex path/to/file.tex` : do not expand `\include`s, `\input`s etc
- `latexpand --expand-usepackage -o path/to/output.tex path/to/file.tex` : expand `\usepackage`s as far as the corresponding STY files can be found
- `latexpand --expand-bbl path/to/bibliography.bbl -o path/to/output.tex path/to/file.tex` : inline the specified BBL file

## Categories
files, documents

## Links
- More information: https://www.ctan.org/pkg/latexpand
- tldr page: https://tldr.inbrowser.app/pages/common/latexpand
