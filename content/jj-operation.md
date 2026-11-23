# jj operation

Work with the operation log of a `jj` repository. Some subcommands such as `abandon`, `diff`, `integrate`, `log`, `restore`, `revert`, and `show` have their own usage documentation.

## Usage
- `jj op log` : show operation log
- `jj op abandon operation` : abandon a specific operation
- `jj op restore operation` : restore the repository to its state at a given operation
- `jj op revert operation` : revert an earlier operation by applying its inverse
- `jj op show operation` : show changes to the repository in an operation
- `jj op show --stat -s -p operation` : show stat, summary, and patch of modifications of an operation
- `jj op diff -f from_op -t to_op` : compare repository changes between two operations
- `jj op diff --stat --op operation_id` : show diff as a histogram of changes

## Categories
monitoring, version-control, packages

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-operation
- tldr page: https://tldr.inbrowser.app/pages/common/jj-operation
