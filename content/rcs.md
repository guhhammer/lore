# rcs

Revision Control System - manage RCS file attributes.

## Usage
- `rcs -u path/to/file` : unlock a revision locked by another user (requires access)
- `rcs -lrevision path/to/file` : lock a specific revision of a file
- `rcs -L path/to/file` : set the strict locking mode (require lock for check-in)
- `rcs -mrevision:"new_message" path/to/file` : add or replace a log message for a specific revision
- `rcs -orevision path/to/file` : delete a specific revision from the RCS file

## See also
- ci
- co
- rcsdiff
- rlog

## Categories
files

## Links
- More information: https://manned.org/rcs
- tldr page: https://tldr.inbrowser.app/pages/common/rcs
- Wikipedia: https://en.wikipedia.org/wiki/Revision_Control_System
