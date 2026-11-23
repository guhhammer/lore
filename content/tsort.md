# tsort

Perform a topological sort. A common use is to show the dependency order of nodes in a directed acyclic graph.

## Usage
- `tsort path/to/file` : perform a topological sort consistent with a partial sort per line of input separated by blanks
- `echo -e "UI Backend\nBackend Database\nDocs UI" | tsort` : perform a topological sort consistent on strings

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/tsort
