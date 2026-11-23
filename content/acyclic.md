# acyclic

Make a directed graph acyclic by reversing some edges. Graphviz filters: `acyclic`, `bcomps`, `comps`, `edgepaint`, `gvcolor`, `gvpack`, `mingle`, `nop`, `sccmap`, `tred`, and `unflatten`.

## Usage
- `acyclic path/to/input.gv >path/to/output.gv` : make a directed graph acyclic by reversing some edges
- `acyclic -v -n path/to/input.gv` : print if a graph is acyclic, has a cycle, or is undirected, producing no output graph
- `acyclic -?` : display help

## Categories
documents, files

## Links
- More information: https://graphviz.org/pdf/acyclic.1.pdf
- tldr page: https://tldr.inbrowser.app/pages/common/acyclic
