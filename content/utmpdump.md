# utmpdump

Dump and load btmp, utmp, and wtmp accounting files.

## Usage
- `utmpdump /var/log/wtmp` : dump the `/var/log/wtmp` file to `stdout` as plain text
- `utmpdump -r dumpfile >/var/log/wtmp` : load a previously dumped file into `/var/log/wtmp`

## Categories
monitoring

## Links
- More information: https://manned.org/utmpdump
- tldr page: https://tldr.inbrowser.app/pages/linux/utmpdump
