# jj abandon

Abandon a revision, rebasing descendants onto its parent(s). Abandoning a revision removes its associated change ID.

## Usage
- `jj abandon revsets` : abandon revisions specified by given revsets (e.g. `B::D`, `A..D`, `B|C|D`, etc.)
- `jj abandon --retain-bookmarks revsets` : abandon revisions, without deleting their bookmarks and moving them to the parent revisions instead
- `jj abandon --restore-descendants revsets` : abandon revisions, without modifying the contents of their children

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-abandon
- tldr page: https://tldr.inbrowser.app/pages/common/jj-abandon
