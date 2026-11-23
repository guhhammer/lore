# jj interdiff

Compare changes of two revisions.

## Usage
- `jj interdiff -f revset` : compare changes from a revision to the working copy
- `jj interdiff -f from_revset -t to_revset` : compare changes from a revision to another revision
- `jj interdiff -f from_revset -t to_revset filesets` : compare changes in specific paths only
- `jj interdiff -f revset -s` : show a summary of changes
- `jj interdiff -f revset --stat` : show diff statistics
- `jj interdiff -f revset --git` : show a Git-format diff
- `jj interdiff -f revset --color-words` : show a word-level diff with changes indicated only by color

## Categories
version-control

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-interdiff
- tldr page: https://tldr.inbrowser.app/pages/common/jj-interdiff
