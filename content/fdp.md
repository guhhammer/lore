# fdp

Render an image of a `force-directed` network graph from a `graphviz` file. Layouts: `dot`, `neato`, `twopi`, `circo`, `fdp`, `sfdp`, `osage`, and `patchwork`.

## Usage
- `fdp -T png -O path/to/input.gv` : render a PNG image with a filename based on the input filename and output format (uppercase -O)
- `fdp -T svg -o path/to/image.svg path/to/input.gv` : render an SVG image with the specified output filename (lowercase -o)
- `fdp -T ps|pdf|svg|fig|png|gif|jpg|json|dot -O path/to/input.gv` : render the output in a specific format
- `echo "digraph {this -> that} " | fdp -T gif >path/to/image.gif` : render a `gif` image using `stdin` and `stdout`
- `fdp -?` : display help

## Categories
media, containers, files

## Links
- More information: https://graphviz.org/doc/info/command.html
- tldr page: https://tldr.inbrowser.app/pages/common/fdp
