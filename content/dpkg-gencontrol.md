# dpkg-gencontrol

Generate Debian control files.

## Usage
- `dpkg-gencontrol [option...]` : the general form, as the manual gives it
- `dpkg-gencontrol -vversion` : sets the version number of the binary package which will be generated
- `dpkg-gencontrol -Vname=value` : set an output substitution variable
- `dpkg-gencontrol -Dfield=value` : override or add an output control file field
- `dpkg-gencontrol -Ufield` : remove an output control file field
