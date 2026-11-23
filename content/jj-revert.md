# jj revert

Apply the reverse of the given revision(s).

## Usage
- `jj revert -r revsets` : apply the reverse of the revisions specified by the given revsets (e.g. `B::D`, `A..D`, `B|C|D`, etc.)
- `jj revert -r revsets -d revsets` : apply the reverse on top of specified revisions
- `jj revert -r revsets -B revsets -A revsets` : apply the reverse before and/or after specified revisions

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-revert
- tldr page: https://tldr.inbrowser.app/pages/common/jj-revert
