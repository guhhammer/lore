# gvmap

Find clusters and creates a geographical map highlighting clusters.

## Usage
- `gvmap graph.gv -o output.xdot` : generate a clustered map layout from a DOT format graph
- `gvmap -D graph.gv` : generate a map using existing cluster subgraphs from the input
- `gvmap -e graph.gv -o output.xdot` : include graph edges in the output map
- `gvmap -c 1|2|3|... graph.gv -o output.xdot` : specify a color scheme (1: pastel, 2: blue-yellow, 3: white-red, etc.)
- `gvmap -C 10 graph.gv -o output.xdot` : set the maximum number of Clusters (by default 0, meaning there is no limit)
- `gvmap -highlight=2 graph.gv -o output.xdot` : only draw cluster 2 (by default, all clusters are drawn)

## Categories
media

## Links
- More information: https://graphviz.org/pdf/gvmap.1.pdf
- tldr page: https://tldr.inbrowser.app/pages/common/gvmap
