# bcomps

Decompose graphs into their biconnected components. Graphviz filters: `acyclic`, `bcomps`, `comps`, `edgepaint`, `gvcolor`, `gvpack`, `mingle`, `nop`, `sccmap`, `tred`, and `unflatten`.

## Usage
- `bcomps path/to/input1.gv path/to/input2.gv ... >path/to/output.gv` : decompose one or more graphs into their biconnected components
- `bcomps -v -s path/to/input1.gv path/to/input2.gv ...` : print the number of blocks and cutvertices in one or more graphs
- `bcomps -x -o path/to/output.gv path/to/input1.gv path/to/input2.gv ...` : write each block and block-cutvertex tree to multiple numbered filenames based on `output.gv`
- `bcomps -?` : display help

## Categories
files, documents

## Links
- More information: https://graphviz.org/pdf/bcomps.1.pdf
- tldr page: https://tldr.inbrowser.app/pages/common/bcomps
