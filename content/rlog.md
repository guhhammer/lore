# rlog

Print the revision log of RCS files.

## Usage
- `rlog path/to/file` : display the complete revision history of a file
- `rlog -h path/to/file` : display only the header information (no revision details)
- `rlog -rrevision path/to/file` : display information for a specific revision
- `rlog -d"start_date<end_date" path/to/file` : display revisions within a date range
- `rlog -wauthor path/to/file` : display revisions by a specific author

## See also
- ci
- co
- rcs
- rcsdiff

## Categories
files, desktop, documents

## Links
- More information: https://manned.org/rlog
- tldr page: https://tldr.inbrowser.app/pages/common/rlog
