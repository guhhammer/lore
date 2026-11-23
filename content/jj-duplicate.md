# jj duplicate

Create new changes with the same content as existing ones.

## Usage
- `jj duplicate` : duplicate the current revision onto its existing parent
- `jj duplicate revset` : duplicate a specific revision onto its existing parent
- `jj duplicate -d dest_revset revset` : duplicate a revision onto a different parent
- `jj duplicate -A after_revset revset` : duplicate a revision and insert it after other revision(s)
- `jj duplicate -B before_revset revset` : duplicate a revision and insert it before other revision(s)
- `jj duplicate -d destination1 -d destination2 revset` : duplicate onto multiple parents (creates a merge commit)
- `jj duplicate revset1 revset2 ...` : duplicate multiple revisions

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-duplicate
- tldr page: https://tldr.inbrowser.app/pages/common/jj-duplicate
