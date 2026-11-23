# osage

Render an image of a `clustered` network graph from a `graphviz` file. Layouts: `dot`, `neato`, `twopi`, `circo`, `fdp`, `sfdp`, `osage` & `patchwork`.

## Usage
- `osage -T png -O path/to/input.gv` : render a PNG image with a filename based on the input filename and output format (uppercase -O)
- `osage -T svg -o path/to/image.svg path/to/input.gv` : render an SVG image with the specified output filename (lowercase -o)
- `osage -T format -O path/to/input.gv` : render the output in PS, PDF, SVG, Fig, PNG, GIF, JPEG, JSON, or DOT format
- `echo "digraph {this -> that} " | osage -T gif >path/to/image.gif` : render a GIF image using `stdin` and `stdout`
- `osage -?` : display help

## Categories
media, containers, files

## Links
- More information: https://graphviz.org/doc/info/command.html
- tldr page: https://tldr.inbrowser.app/pages/common/osage
