# systemd-hwdb

Hardware database management tool.

## Usage
- `sudo systemd-hwdb update` : update the binary hardware database in `/etc/udev`
- `systemd-hwdb query modalias` : query the hardware database and print the result for a specific modalias
- `sudo systemd-hwdb -s update` : update the binary hardware database, returning a non-zero exit value on any parsing error
- `sudo systemd-hwdb --usr update` : update the binary hardware database in `/usr/lib/udev`
- `systemd-hwdb -r path/to/root update` : update the binary hardware database in the specified root path

## Categories
system, databases, users

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemd-hwdb.html
- tldr page: https://tldr.inbrowser.app/pages/linux/systemd-hwdb
