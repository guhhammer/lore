# jj rebase

Move revisions to different parent(s).

## Usage
- `jj rebase -r revset -o revset` : move given revisions to a different parent(s)
- `jj rebase -s revset -o revset` : move given revisions and all their descendants
- `jj rebase -b revset -o revset` : move all revisions in the branch containing given revisions
- `jj rebase -r revset -B revset -A revset` : move revisions to before and/or after other revisions

## Categories
files

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-rebase
- tldr page: https://tldr.inbrowser.app/pages/common/jj-rebase
