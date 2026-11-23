# systemctl switch-root

Switch to a new root filesystem and execute a new system manager.

## Usage
- `systemctl switch-root path/to/new_root` : switch to a new root filesystem and execute its default init system
- `systemctl switch-root path/to/new_root /sbin/init` : switch to a new root filesystem and run a specific init binary
- `systemctl switch-root path/to/new_root -v` : switch to a new root filesystem with verbose output

## Categories
disks, files

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-switch-root
