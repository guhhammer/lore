# gvcolor

Colorize a ranked digraph with a range of colors. Graphviz filters: `acyclic`, `bcomps`, `comps`, `edgepaint`, `gvcolor`, `gvpack`, `mingle`, `nop`, `sccmap`, `tred`, and `unflatten`.

## Usage
- `gvcolor path/to/layout1.gv path/to/layout2.gv ... >path/to/output.gv` : colorize one or more ranked digraph (that were already processed by `dot`)
- `dot path/to/input.gv | gvcolor | dot -T png >path/to/output.png` : lay out a graph and colorize it, then convert to a PNG image
- `gvcolor -?` : display help

## Categories
files, media

## Links
- More information: https://graphviz.org/pdf/gvcolor.1.pdf
- tldr page: https://tldr.inbrowser.app/pages/common/gvcolor
