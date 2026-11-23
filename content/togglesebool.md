# togglesebool

Flip the current (non-persistent) values of SELinux booleans. Note: This command is deprecated, use `setsebool` instead.

## Usage
- `sudo togglesebool virt_use_samba|virt_use_usb|...` : flip the current (non-persistent) values of the specified booleans
- `sudo togglesebool key1 key2 ...` : flip multiple booleans

## Links
- More information: https://manned.org/togglesebool
- tldr page: https://tldr.inbrowser.app/pages/linux/togglesebool
