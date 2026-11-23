# whereis

Locate the binary, source, and manual page files for a command.

## Usage
- `whereis ssh` : locate binary, source, and man pages for SSH
- `whereis -bm ls` : locate binary and man pages for ls
- `whereis -s gcc -m git` : locate source of gcc and man pages for Git
- `whereis -b -B /usr/bin/ -f gcc` : locate binaries for gcc in `/usr/bin/` only
- `whereis -u *` : locate unusual binaries (those that have more or less than one binary on the system)
- `whereis -u -m *` : locate binaries that have unusual manual entries (binaries that have more or less than one manual installed)

## See also
- which
- whatis
- type

## Categories
network, documents

## Links
- More information: https://manned.org/whereis
- tldr page: https://tldr.inbrowser.app/pages/linux/whereis
