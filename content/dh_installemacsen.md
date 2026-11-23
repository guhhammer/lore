# dh_installemacsen

Register an Emacs add on package.

## Usage
- `dh_installemacsen [debhelper options] [-n] [--priority=n] [--flavor=foo]` : the general form, as the manual gives it
- `dh_installemacsen -n` : do not modify postinst/prerm scripts
- `dh_installemacsen --priority=n` : sets the priority number of a site-start.d file
- `dh_installemacsen --flavor=foo` : sets the flavor a site-start.d file will be installed in

## Categories
editors, packages, processes
