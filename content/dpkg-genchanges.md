# dpkg-genchanges

Generate Debian .changes files.

## Usage
- `dpkg-genchanges [option...]` : the general form, as the manual gives it
- `dpkg-genchanges -g` : equivalent to --build=source,all (since dpkg 1.17.11)
- `dpkg-genchanges -G` : equivalent to --build=source,any (since dpkg 1.17.11)
- `dpkg-genchanges -b` : equivalent to --build=binary or --build=any,all
- `dpkg-genchanges -B` : equivalent to --build=any

## Categories
development
