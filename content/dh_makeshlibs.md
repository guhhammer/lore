# dh_makeshlibs

Automatically create shlibs file and call dpkg-gensymbols.

## Usage
- `dh_makeshlibs [debhelper options] [-mmajor] [-V[dependencies]] [-n] [-Xitem] [-- params]` : the general form, as the manual gives it
- `dh_makeshlibs --no-add-udeb` : do not add any udeb lines to the shlibs file

## Categories
files
