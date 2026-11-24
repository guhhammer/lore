# checkinstall

Track the local installation of a software package, and produce a binary package which can be used with a system's native package manager.

## Usage
- `sudo checkinstall -y` : create and install a package with default settings
- `sudo checkinstall --install=no` : create a package but don't install it
- `sudo checkinstall --nodoc` : create a package without documentation
- `sudo checkinstall --pkgname package` : create a package and set the name
- `sudo checkinstall --pakdir path/to/directory` : create a package and specify where to save it

## Categories
packages, users

## Links
- More information: https://checkinstall.izto.org/docs.php
- tldr page: https://tldr.inbrowser.app/pages/linux/checkinstall
- Wikipedia: https://en.wikipedia.org/wiki/CheckInstall
