# dmesg

Write the kernel messages to `stdout`.

## Usage
- `sudo dmesg` : show kernel messages
- `sudo dmesg -H` : show kernel messages in human-readable format (equivalent of `dmesg --color --reltime` piped to a pager)
- `sudo dmesg -l err` : show kernel error messages
- `sudo dmesg -w` : show kernel messages and keep waiting for new ones (similar to `tail --follow`)
- `sudo dmesg --since "1 hour ago"` : show kernel messages emitted in the last hour
- `sudo dmesg -e` : show kernel messages with timestamps as differences from local time
- `sudo dmesg -T` : show kernel messages with a timestamp for each message
- `sudo dmesg -L` : colorize output

## See also
- journalctl

## Categories
system, hardware

## Links
- More information: https://manned.org/dmesg
- tldr page: https://tldr.inbrowser.app/pages/linux/dmesg
