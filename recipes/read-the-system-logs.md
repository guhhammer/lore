# read the system logs

Find what went wrong: the errors since boot, a service's log, the kernel's messages.

## Steps
- `journalctl -b -p err` : every error since this boot
- `journalctl -xe` : the latest entries, explained
- `journalctl --since '1 hour ago'` : the last hour
- `journalctl -u NAME` : one service's log
- `sudo dmesg -T | tail -50` : the kernel's latest messages: disks, USB, drivers
- `journalctl -b -1 -p err` : the errors of the boot before, after a crash

## Notes
- Without systemd's journal: /var/log/syslog or /var/log/messages.

## See also
- journalctl
- dmesg

## Categories
system services
