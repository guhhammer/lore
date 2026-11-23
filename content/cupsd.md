# cupsd

Server daemon for the CUPS print server.

## Usage
- `cupsd` : start `cupsd` in the background, aka. as a daemon
- `cupsd -f` : start `cupsd` on the foreground
- `cupsd -l` : launch `cupsd` on-demand (commonly used by `launchd` or `systemd`)
- `cupsd -c path/to/cupsd.conf` : start `cupsd` using the specified `cupsd.conf` configuration file
- `cupsd -s path/to/cups-files.conf` : start `cupsd` using the specified `cups-files.conf` configuration file
- `cupsd -t -c path/to/cupsd.conf` : test the `cupsd.conf` configuration file for errors
- `cupsd -t -s path/to/cups-files.conf` : test the `cups-files.conf` configuration file for errors
- `cupsd -h` : display help

## Categories
files, system, documents

## Links
- More information: https://openprinting.github.io/cups/doc/man-cupsd.html
- tldr page: https://tldr.inbrowser.app/pages/common/cupsd
