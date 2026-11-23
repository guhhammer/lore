# systemd-cgtop

Show the top control groups of the local Linux control group hierarchy, ordered by their CPU, memory, or disk I/O load.

## Usage
- `systemd-cgtop` : start an interactive view
- `systemd-cgtop --order cpu|memory|path|tasks|io` : change the sort order
- `systemd-cgtop --cpu=percentage` : show the CPU usage by time instead of percentage
- `systemd-cgtop -d interval` : change the update interval in seconds (or one of these time units: `ms`, `us`, `min`)
- `systemd-cgtop -P` : only count userspace processes (without kernel threads)

## See also
- top

## Categories
system, monitoring, hardware

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemd-cgtop.html
- tldr page: https://tldr.inbrowser.app/pages/linux/systemd-cgtop
