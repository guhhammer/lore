# jj operation abandon

Abandon operation history in a `jj` repository.

## Usage
- `jj op abandon operation_id` : abandon a specific operation
- `jj op abandon ..operation_id` : abandon a specific operation and all of its ancestors
- `jj op abandon operation_id..@-` : discard recent operations after restoring to an earlier state

## See also
- jj operation log
- jj operation restore
- jj operation revert

## Categories
packages, shell

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-operation-abandon
- tldr page: https://tldr.inbrowser.app/pages/common/jj-operation-abandon
