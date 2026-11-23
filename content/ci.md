# ci

Check in RCS revisions (store file changes in the Revision Control System).

## Usage
- `ci -u path/to/file` : check in a file and keep the working file unlocked
- `ci -l path/to/file` : check in a file and keep the working file locked
- `ci -m"log_message" path/to/file` : check in a file with a specific log message
- `ci path/to/file` : check in a file, unlocking it but leaving the working file read-only
- `ci -f -u path/to/file` : force check-in even if there are no changes

## See also
- co
- rcs
- rcsdiff
- rlog

## Categories
files

## Links
- More information: https://manned.org/ci
- tldr page: https://tldr.inbrowser.app/pages/common/ci
