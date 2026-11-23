# jj restore

Restore files from another revision.

## Usage
- `jj restore -f revset -t revset filesets` : restore files from a revision into another revision
- `jj restore -c revset filesets` : undo the changes in a revision as compared to the merge of its parents
- `jj restore -f revset -t revset -i` : interactively choose what parts to restore

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-restore
- tldr page: https://tldr.inbrowser.app/pages/common/jj-restore
