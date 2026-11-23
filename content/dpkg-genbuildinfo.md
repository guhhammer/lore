# dpkg-genbuildinfo

Generate Debian .buildinfo files.

## Usage
- `dpkg-genbuildinfo [option...]` : the general form, as the manual gives it
- `dpkg-genbuildinfo --build=type` : specifies the build type from a comma-separated list of components
- `dpkg-genbuildinfo -ccontrolfile` : specifies the main source control file to read information from
- `dpkg-genbuildinfo -lchangelog-file` : specifies the changelog file to read information from
- `dpkg-genbuildinfo -Fchangelog-format` : specifies the format of the changelog

## Categories
files
