# jj operation restore

Restore the repository to an earlier state in the operation log.

## Usage
- `jj op restore operation_id` : restore the repository state at a specific operation
- `jj op restore --what repo operation_id` : restore only the repository state and local bookmarks (ignoring remote-tracking bookmarks)

## See also
- jj undo
- jj operation log
- jj operation revert
- jj operation show

## Categories
packages, monitoring

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-operation-restore
- tldr page: https://tldr.inbrowser.app/pages/common/jj-operation-restore
