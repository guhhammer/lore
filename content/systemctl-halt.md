# systemctl-halt

Shut down and halt the system (stop the OS kernel but keep hardware powered on).

## Usage
- `systemctl halt` : halt the system
- `systemctl halt -f` : halt the system immediately without asking services to stop gracefully
- `systemctl halt -f --no-wall` : halt the system immediately without sending notifications to logged-in users
- `systemctl halt -ff` : halt the system immediately without terminating any processes or unmounting filesystems (dangerous, may cause data loss)
- `systemctl halt --when 23:00` : schedule a halt at a specific time (e.g., 23:00)
- `systemctl halt --when +2h` : schedule a halt after a certain duration (e.g., 2 hours)
- `systemctl halt --when cancel` : cancel a scheduled halt

## See also
- halt

## Categories
system

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemctl.html#halt
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-halt
