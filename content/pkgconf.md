# pkgconf

A system for configuring build dependency information.

## Usage
- `pkgconf [options] [list of modules]` : the general form, as the manual gives it
- `pkgconf --version` : display the supported pkg-config version and exit
- `pkgconf --atleast-pkgconfig-version=VERSION` : exit with error if we do not support the requested pkg-config version
- `pkgconf --silence-errors` : do not display any errors at all
- `pkgconf --no-cache` : skip caching packages when they are loaded into the internal resolver

## Categories
development
