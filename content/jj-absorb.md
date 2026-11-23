# jj absorb

Split changes in the source revision and move each change to the closest mutable ancestor where the corresponding lines were modified last. Changes that have zero or multiple matching regions in ancestral revisions won't be moved.

## Usage
- `jj absorb -f revset -t revsets` : move all eligible and unambiguous changes from a revision to other revisions automatically
- `jj absorb -f revset -t revsets filesets` : move only changes in given files from a revision to other revisions

## Categories
files, text

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-absorb
- tldr page: https://tldr.inbrowser.app/pages/common/jj-absorb
