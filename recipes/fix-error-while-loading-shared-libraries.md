# fix error while loading shared libraries

"cannot open shared object file": a program needs a library that is missing or not where it looks.

## Steps
- `ldd /path/to/program` : every library it needs; the missing ones say "not found"
- `ldconfig -p | grep libNAME` : is the library known to the system?
- `apt-file search libNAME.so.1` : the package that has it (Debian, Ubuntu; dnf provides on Fedora)
- `sudo ldconfig` : after installing a library in /usr/local/lib, let the system find it
- `LD_LIBRARY_PATH=/opt/app/lib /path/to/program` : run it with an extra folder to look in

## See also
- ldd
- ldconfig
- apt-file

## Categories
development packages
