# mingle

Bundle the edges of a graph layout. Graphviz filters: `acyclic`, `bcomps`, `comps`, `edgepaint`, `gvcolor`, `gvpack`, `mingle`, `nop`, `sccmap`, `tred`, and `unflatten`.

## Usage
- `mingle path/to/layout1.gv path/to/layout2.gv ... >path/to/output.gv` : bundle the edges of one or more graph layouts (that already have layout information)
- `dot path/to/input.gv | mingle | dot -T png >path/to/output.png` : perform layout, bundling, and output to a picture with one command
- `mingle -?` : display help

## Categories
files, media

## Links
- More information: https://www.graphviz.org/pdf/mingle.1.pdf
- tldr page: https://tldr.inbrowser.app/pages/common/mingle
