# dpkg-buildpackage

Compile binary and/or source Debian packages from source code. Normally executed inside a source tree that contains a `debian/` directory. Also handles build dependencies, creates files like `.buildinfo` and `.changes`, and signs the result if applicable.

## Usage
- `dpkg-buildpackage` : generate source and binary packages
- `dpkg-buildpackage -b` : generate only binary packages (no source package)
- `dpkg-buildpackage -S` : generate only the source package (without compiling binaries)
- `dpkg-buildpackage -us -uc` : do not sign the `.dsc` and `.changes` files
- `dpkg-buildpackage -nc` : do not run `clean` before compiling
- `dpkg-buildpackage -rfakeroot` : use `fakeroot` as the command to gain root privileges during the build
- `dpkg-buildpackage -Tclean` : run a specific `debian/rules` target

## Categories
development, packages, files

## Links
- More information: https://manned.org/dpkg-buildpackage
- tldr page: https://tldr.inbrowser.app/pages/linux/dpkg-buildpackage
