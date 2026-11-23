# jj split

Split a revision in two.

## Usage
- `jj split -r revision` : split the given revision into two interactively, putting the second revision on top of it
- `jj split -r revision fileset` : split out matching files from the given revision
- `jj split -r revision -d revset` : split the given revision, putting the second revision on top of given destination(s)
- `jj split -r revision -B revset -A revset` : split the given revision, putting the second revision before and/or after other revision(s)
- `jj split -r revision -p` : split the given revision into two parallel revisions

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-split
- tldr page: https://tldr.inbrowser.app/pages/common/jj-split
