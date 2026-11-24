# ldconfig

Configure symlinks and cache for shared library dependencies.

## Usage
- `sudo ldconfig` : update symlinks and rebuild the cache (usually run when a new library is installed)
- `sudo ldconfig -n path/to/directory` : update the symlinks for a given directory
- `ldconfig -p | grep library_name` : print the libraries in the cache and check whether a given library is present

## Categories
development, files, packages

## Links
- More information: https://manned.org/ldconfig
- tldr page: https://tldr.inbrowser.app/pages/linux/ldconfig
- Wikipedia: https://en.wikipedia.org/wiki/Ldconfig
