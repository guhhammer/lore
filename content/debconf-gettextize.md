# debconf-gettextize

Extract translations of debconf templates into PO files.

## Usage
- `debconf-gettextize [-v] [-h] [--podir=DIR] [--choices] [--merge] master [master ...]` : the general form, as the manual gives it
- `debconf-gettextize -h` : display a usage summary for the program and exit
- `debconf-gettextize -v` : process in verbose mode
- `debconf-gettextize --podir=DIR` : set directory for PO files
- `debconf-gettextize --merge` : when this flag is set, the strings are merged with existing PO files

## Categories
archives
