# chroma

A general-purpose syntax highlighter. The `--lexer` option is usually unnecessary, as it will be automatically determined based on the file extension.

## Usage
- `chroma -l python path/to/source_file.py` : highlight source code from a file with the Python lexer and output to `stdout`
- `chroma -l go -f html path/to/source_file.go >path/to/target_file.html` : highlight source code from a file with the Go lexer and output to an HTML file
- `command | chroma -l c++ -f svg -s monokai >path/to/target_file.svg` : highlight source code from `stdin` with the C++ lexer and output to an SVG file, using the Monokai style
- `chroma --list` : list available lexers, styles, and formatters

## Categories
files, development

## Links
- More information: https://manned.org/chroma
- tldr page: https://tldr.inbrowser.app/pages/common/chroma
