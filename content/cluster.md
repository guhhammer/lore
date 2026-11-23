# cluster

Find clusters in a graph and augment the graph with this information.

## Usage
- `cluster input.gv` : generate clusters that optimize modularity and print the result to `stdout`
- `cluster -C 5 input.gv` : specify a target number of Clusters (approximate) to generate (0 by default, meaning a number that approximately optimizes the modularity)
- `cluster -c 0|1 input.gv` : use a different clustering method (0: modularity clustering, 1: modularity quality)
- `cluster -o output.gv input.gv` : save the output to a file
- `cluster -v input.gv` : enable verbose mode

## Categories
containers, documents

## Links
- More information: https://graphviz.org/pdf/cluster.1.pdf
- tldr page: https://tldr.inbrowser.app/pages/common/cluster
