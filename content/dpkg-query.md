# dpkg-query

Display information about installed packages.

## Usage
- `dpkg-query -l` : list all installed packages
- `dpkg-query -l 'libc6*'` : list installed packages matching a pattern
- `dpkg-query -L libc6` : list all files installed by a package
- `dpkg-query -s libc6` : show information about a package
- `dpkg-query -S /etc/ld.so.conf.d` : search for packages that own files matching a pattern

## Categories
databases, packages, desktop

## Links
- More information: https://manned.org/dpkg-query
- tldr page: https://tldr.inbrowser.app/pages/linux/dpkg-query
