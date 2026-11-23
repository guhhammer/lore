# ccomps

Decompose graphs into their connected components. Graphviz filters: `acyclic`, `bcomps`, `comps`, `edgepaint`, `gvcolor`, `gvpack`, `mingle`, `nop`, `sccmap`, `tred`, and `unflatten`.

## Usage
- `ccomps path/to/input1.gv path/to/input2.gv ... >path/to/output.gv` : decompose one or more graphs into their connected components
- `ccomps -v -s path/to/input1.gv path/to/input2.gv ...` : print the number of nodes, edges, and connected components in one or more graphs
- `ccomps -x -o path/to/output.gv path/to/input1.gv path/to/input2.gv ...` : write each connected component to numbered filenames based on `output.gv`
- `ccomps -?` : display help

## Categories
files, documents

## Links
- More information: https://graphviz.org/pdf/ccomps.1.pdf
- tldr page: https://tldr.inbrowser.app/pages/common/ccomps
