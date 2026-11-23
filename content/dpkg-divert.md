# dpkg-divert

Manages diversions: redirects where a package's file is installed so a local version is not overwritten on upgrade.

## Usage
- `dpkg-divert --list` : show every diversion in place
- `dpkg-divert --add --rename /usr/bin/tool` : move a file aside and divert future installs of it
- `dpkg-divert --remove --rename /usr/bin/tool` : undo a diversion and move the file back
- `dpkg-divert --list '*tool*'` : show only diversions matching a pattern

## Notes
Needs root. Remove a diversion before removing the package that owns the file, or the package system gets confused.

## Categories
files, packages
