# jj arrange

Interactively arrange the commit graph.

## Usage
- `jj arrange` : interactively arrange commits configured by `revsets.arrange` (defaults to mutable commits)
- `jj arrange -r revset` : interactively arrange specific revisions
- `jj arrange -r revset1 -r revset2` : interactively arrange multiple revsets
- `jj arrange -r 'trunk()..@'` : interactively arrange a stack of commits from `trunk` to the current revision

## See also
- jj rebase
- jj squash

## Categories
version-control

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-arrange
- tldr page: https://tldr.inbrowser.app/pages/common/jj-arrange
