# find

Looks for files and folders under a folder, by name, type, size, age and
more, and can run a command on each one found.

## Usage
- `find . -name '*.pdf'` : every PDF under the current folder
- `find /var/log -mtime -1` : files changed in the last day
- `find . -type d -name node_modules` : folders with that name
- `find . -size +100M` : files bigger than 100 MB
- `find . -name '*.tmp' -delete` : delete what it finds (check first without -delete)

## See also
- fd

## Categories
files

## Links
- GNU manual: https://www.gnu.org/software/findutils/find
