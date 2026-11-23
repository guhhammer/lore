# dpkg-deb

Pack, unpack, and provide information about Debian archives.

## Usage
- `dpkg-deb -I path/to/file.deb` : display information about a package
- `dpkg-deb -W path/to/file.deb` : display the package's name and version on one line
- `dpkg-deb -c path/to/file.deb` : list the package's contents
- `dpkg-deb -x path/to/file.deb path/to/directory` : extract package's contents into a directory
- `dpkg-deb -b path/to/directory` : create a package from a specified directory

## Categories
files, packages

## Links
- More information: https://manned.org/dpkg-deb
- tldr page: https://tldr.inbrowser.app/pages/linux/dpkg-deb
