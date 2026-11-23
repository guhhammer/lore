# dh_installinit

Install service init files into package build directories.

## Usage
- `dh_installinit [debhelper options] [--name=name] [-n] [-R] [-r] [-d] [-- params]` : the general form, as the manual gives it
- `dh_installinit -n` : do not modify postinst/postrm/prerm scripts
- `dh_installinit -r` : do not stop init script on upgrade
- `dh_installinit --no-enable` : disable the init script on purge, but do not enable them on install
- `dh_installinit --error-handler=function` : call  the  named  shell  function  if  running  the init script fails

## Categories
packages, development, files
