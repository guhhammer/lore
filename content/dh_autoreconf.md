# dh_autoreconf

Call autoreconf -f -i and keep track of the changed files.

## Usage
- `dh_autoreconf [debhelper options] [-Xitem] [--mode=mode] [program -- params]` : the general form, as the manual gives it
- `dh_autoreconf --mode=mode` : change  the  way in which modifications to files are detected
