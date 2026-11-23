# co

Check out RCS revisions (retrieve working files from the Revision Control System).

## Usage
- `co path/to/file` : check out the latest revision of a file (retrieves a read-only copy)
- `co -l path/to/file` : check out a file with a lock for editing
- `co -rrevision path/to/file` : check out a specific revision of a file
- `co -f -l path/to/file` : check out a file and overwrite it if it already exists
- `co -p -rrevision path/to/file` : print a specific revision to `stdout` without creating a file

## See also
- ci
- rcs
- rcsdiff
- rlog

## Categories
files

## Links
- More information: https://manned.org/co
- tldr page: https://tldr.inbrowser.app/pages/common/co
