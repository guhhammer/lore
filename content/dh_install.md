# dh_install

Install files into package build directories.

## Usage
- `dh_install [-Xitem] [--autodest] [--sourcedir=dir] [debhelper options] [file|dir ... destdir]` : the general form, as the manual gives it
- `dh_install --list-missing` : deprecated: Please use dh_missing --list-missing instead
- `dh_install --fail-missing` : deprecated:  Please  use  dh_missing  --fail-missing instead
- `dh_install --sourcedir=dir` : look in the specified directory for files to be installed
- `dh_install --autodest` : guess as the destination directory to install things to

## Categories
packages, files, development
