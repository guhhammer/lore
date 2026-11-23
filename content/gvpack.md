# gvpack

Combine several graph layouts (that already have layout information). Graphviz filters: `acyclic`, `bcomps`, `comps`, `edgepaint`, `gvcolor`, `gvpack`, `mingle`, `nop`, `sccmap`, `tred`, and `unflatten`.

## Usage
- `gvpack path/to/layout1.gv path/to/layout2.gv ... >path/to/output.gv` : combine several graph layouts (that already have layout information)
- `gvpack -g path/to/layout1.gv path/to/layout2.gv ... >path/to/output.gv` : combine several graph layouts at the graph level, keeping graphs separate
- `gvpack -n path/to/layout1.gv path/to/layout2.gv ... >path/to/output.gv` : combine several graph layouts at the node level, ignoring clusters
- `gvpack -u path/to/layout1.gv path/to/layout2.gv ... >path/to/output.gv` : combine several graph layouts without packing
- `gvpack -?` : display help

## Categories
files

## Links
- More information: https://graphviz.org/pdf/gvpack.1.pdf
- tldr page: https://tldr.inbrowser.app/pages/common/gvpack
