# journalctl

Query the systemd journal.

## Usage
- `journalctl -n n -f` : show the latest `n` lines and follow new messages (like `tail --follow` for traditional syslog)
- `journalctl -b -1 -p 3` : show all messages with priority level 3 (errors) from the boot before last shutdown
- `journalctl -u unit` : show all messages by a specific unit
- `journalctl _SYSTEMD_INVOCATION_ID=$(systemctl show --value --property=InvocationID unit)` : show logs for a given unit since the last time it started
- `journalctl -S now|today|yesterday|tomorrow|... -U "YYYY-MM-DD HH:MM:SS"` : filter messages within a time range (either timestamp or placeholders like "yesterday")
- `journalctl _PID=process_id` : show all messages by a specific process
- `journalctl path/to/executable` : show all messages by a specific executable
- `journalctl --vacuum-time 2d` : delete journal logs which are older than 2 days

## See also
- dmesg

## Categories
system, monitoring

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/journalctl.html
- tldr page: https://tldr.inbrowser.app/pages/linux/journalctl
