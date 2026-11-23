# rcsmerge

Merge RCS revisions into the working file.

## Usage
- `rcsmerge -rrevision1 -rrevision2 path/to/file` : merge differences between two revisions into the working file
- `rcsmerge -rbranch_revision path/to/file` : merge changes from a branch revision into the working file
- `rcsmerge -q -rrevision1 -rrevision2 path/to/file` : perform a quiet merge (suppress diagnostics)
- `rcsmerge -p -rrevision1 -rrevision2 path/to/file` : print the result to `stdout` instead of overwriting the working file

## See also
- ci
- co
- rcs
- rcsdiff
- rlog

## Categories
files, version-control

## Links
- More information: https://manned.org/rcsmerge
- tldr page: https://tldr.inbrowser.app/pages/common/rcsmerge
