# jj operation diff

Compare changes to the repository between two operations.

## Usage
- `jj op diff --op operation_id` : compare repository changes in a specific operation to its parent
- `jj op diff -f from_op -t to_op` : compare repository changes between two specific operations
- `jj op diff -p --op operation_id` : show diff with a patch of modifications to changes
- `jj op diff --stat --op operation_id` : show diff as a histogram of changes
- `jj op diff -G --op operation_id` : show diff without a graph

## Categories
version-control, packages

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-operation-diff
- tldr page: https://tldr.inbrowser.app/pages/common/jj-operation-diff
