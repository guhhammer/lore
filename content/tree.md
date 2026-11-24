# tree

Show the contents of the current directory as a tree.

## Usage
- `tree -L num` : print files and directories up to `num` levels of depth (where 1 means the current directory)
- `tree -d` : print directories only
- `tree -a -C` : print hidden files too with colorization on
- `tree -i -f` : print the tree without indentation lines, showing the full path instead (use `-N` to not escape non-printable characters)
- `tree -s -h --du` : print the size of each file and the cumulative size of each directory, in human-readable format
- `tree -P '*.txt' --prune` : print files within the tree hierarchy, using a wildcard (`glob`) pattern, and pruning out directories that don't contain matching files
- `tree -P directory_name --matchdirs --prune` : print directories within the tree hierarchy, using the wildcard (`glob`) pattern, and pruning out directories that aren't ancestors of the wanted one
- `tree -I 'directory_name1|directory_name2'` : print the tree ignoring the given directories

## Categories
files

## Links
- More information: https://manned.org/tree
- tldr page: https://tldr.inbrowser.app/pages/common/tree
- Wikipedia: https://en.wikipedia.org/wiki/Tree_(command)
