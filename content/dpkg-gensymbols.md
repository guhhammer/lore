# dpkg-gensymbols

Generate symbols files (shared library dependency information).

## Usage
- `dpkg-gensymbols [option...]` : the general form, as the manual gives it
- `dpkg-gensymbols -Ppackage-build-dir` : scan package-build-dir instead of debian/tmp
- `dpkg-gensymbols -ppackage` : define the package name
- `dpkg-gensymbols -vversion` : define the package version
- `dpkg-gensymbols -aarch` : assume arch as host architecture when processing symbol files

## Categories
development, packages

## Links
- Documentation: https://people.redhat.com/drepper/symbol-versioning
