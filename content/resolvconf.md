# resolvconf

Manage nameserver information. Acts as an intermediary between programs that supply nameserver information and applications that use this information. This page documents Debian's implementation of `resolvconf`.

## Usage
- `resolvconf -a IFACE.PROG` : add or override the IFACE.PROG record and run the update scripts if updating is enabled
- `resolvconf -d IFACR.PROG` : delete the IFACE.PROG record and run the update scripts if updating is enabled
- `resolvconf -u` : just run the update scripts if updating is enabled
- `resolvconf --enable-updates` : set the flag indicating whether `resolvconf` should run update scripts when invoked with `-a`, `-d`, or `-u`
- `resolvconf --disable-updates` : clear the flag indicating whether to run updates
- `resolvconf --updates-are-enabled` : check whether updates are enabled

## Links
- More information: https://manned.org/resolvconf
- tldr page: https://tldr.inbrowser.app/pages/common/resolvconf
