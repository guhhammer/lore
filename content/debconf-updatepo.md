# debconf-updatepo

Update PO files about debconf templates.

## Usage
- `debconf-updatepo [-v] [-h] [--podir=DIR] [--skip-pot] [--skip-merge] [--msgid-bugs-address=EMAIL]` : the general form, as the manual gives it
- `debconf-updatepo -h` : display a usage summary and exit
- `debconf-updatepo -v` : process in verbose mode
- `debconf-updatepo --podir=DIR` : set directory for PO files
- `debconf-updatepo --skip-pot` : do not generate a new po/templates.pot file
