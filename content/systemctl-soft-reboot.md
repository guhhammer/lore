# systemctl soft-reboot

Shut down and reboot userspace, leaving the kernel running.

## Usage
- `systemctl soft-reboot` : perform a soft reboot immediately
- `systemctl soft-reboot -f` : force a soft reboot
- `systemctl soft-reboot --when "timestamp"` : schedule a soft reboot for a specific time
- `systemctl soft-reboot --when cancel` : cancel a scheduled soft reboot

## Categories
system

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-soft-reboot
