# nop

Check validity and pretty-print graphs in canonical format. Graphviz filters: `acyclic`, `bcomps`, `comps`, `edgepaint`, `gvcolor`, `gvpack`, `mingle`, `nop`, `sccmap`, `tred`, and `unflatten`.

## Usage
- `nop path/to/input1.gv path/to/input2.gv ... >path/to/output.gv` : pretty-print one or more graphs in canonical format
- `nop -p path/to/input1.gv path/to/input2.gv ...` : check one or more graphs for validity, producing no output graph
- `nop -?` : display help

## Categories
documents, files

## Links
- More information: https://www.graphviz.org/pdf/nop.1.pdf
- tldr page: https://tldr.inbrowser.app/pages/common/nop
