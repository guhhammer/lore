# dpkg-parsechangelog

Parse Debian changelog files.

## Usage
- `dpkg-parsechangelog [option...]` : the general form, as the manual gives it
- `dpkg-parsechangelog -F changelog-format` : specifies  the format of the changelog
- `dpkg-parsechangelog --version` : show the version and exit
- `dpkg-parsechangelog --format output-format` : set the output format
- `dpkg-parsechangelog --reverse` : include all changes in reverse order (since dpkg 1.19.1)
