# unflatten

Adjust directed graphs to improve the layout aspect ratio. Graphviz filters: `acyclic`, `bcomps`, `comps`, `edgepaint`, `gvcolor`, `gvpack`, `mingle`, `nop`, `sccmap`, `tred`, and `unflatten`.

## Usage
- `unflatten path/to/input1.gv path/to/input2.gv ... >path/to/output.gv` : adjust one or more directed graphs to improve the layout aspect ratio
- `unflatten path/to/input.gv | dot -T png path/to/output.png` : use `unflatten` as a preprocessor for `dot` layout to improve aspect ratio
- `unflatten -?` : display help

## Categories
files

## Links
- More information: https://www.graphviz.org/pdf/unflatten.1.pdf
- tldr page: https://tldr.inbrowser.app/pages/common/unflatten
