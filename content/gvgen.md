# gvgen

Generate simple, structured abstract graphs.

## Usage
- `gvgen -c 10` : generate a cycle with 10 vertices and edges and write it to `stdout`
- `gvgen -g 4,3` : generate a 4×3 grid
- `gvgen -t 5` : generate a binary tree of height 5
- `gvgen -b 3,4` : generate a complete bipartite graph with 3 and 4 vertices
- `gvgen -r 10,5 -o random.gv` : create a random graph and output it to a file
- `gvgen -d -v -c 6` : generate a directed graph with verbose output
- `gvgen -?` : display help

## Links
- More information: https://graphviz.org/pdf/gvgen.1.pdf
- tldr page: https://tldr.inbrowser.app/pages/common/gvgen
