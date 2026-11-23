# extundelete

Recover deleted files from ext3 or ext4 partitions by parsing the journal.

## Usage
- `sudo extundelete /dev/sdXN --restore-all` : restore all deleted files inside partition `N` on device `X`
- `extundelete /dev/sdXN --restore-file path/to/file` : restore a file from a path relative to root (Do not start the path with `/`)
- `extundelete /dev/sdXN --restore-directory path/to/directory` : restore a directory from a path relative to root (Do not start the path with `/`)
- `extundelete /dev/sdXN --restore-all --after 1577840400` : restore all files deleted after January 1st, 2020 (in Unix time)

## See also
- date
- umount

## Categories
files, system

## Links
- More information: https://extundelete.sourceforge.net/options.html
- tldr page: https://tldr.inbrowser.app/pages/linux/extundelete
