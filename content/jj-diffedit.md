# jj diffedit

Touch up the content changes in a revision with a diff editor.

## Usage
- `jj diffedit` : edit changes in the current revision with a diff editor
- `jj diffedit -r revset` : edit changes in a given revision
- `jj diffedit -f from_revset -t to_revset` : edit changes comparing a "from" revision to a "to" revision
- `jj diffedit filesets` : edit only specific paths (unmatched paths remain unchanged)
- `jj diffedit --tool name` : use a specific diff editor
- `jj diffedit --restore-descendants` : preserve content instead of diff when rebasing descendants

## Categories
version-control, editors

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-diffedit
- tldr page: https://tldr.inbrowser.app/pages/common/jj-diffedit
