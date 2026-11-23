# dh_strip

Strip executables, shared libraries, and some static libraries.

## Usage
- `dh_strip [debhelper options] [-Xitem] [--dbg-package=package] [--keep-debug]` : the general form, as the manual gives it
- `dh_strip --ddebs` : historical name for --automatic-dbgsym and --no-automatic-dbgsym
- `dh_strip --ddeb-migration=package-relation` : historical name for --dbgsym-migration

## Categories
packages
