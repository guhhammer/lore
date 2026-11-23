# systemd-firstboot

Initialize basic system settings on or before the first boot-up of a system.

## Usage
- `sudo systemd-firstboot --root path/to/root_directory` : operate on the specified directory instead of the root directory of the host system
- `sudo systemd-firstboot --keymap keymap` : set the system keyboard layout
- `sudo systemd-firstboot --hostname hostname` : set the system hostname
- `sudo systemd-firstboot --root-password password` : set the root user's password
- `sudo systemd-firstboot --prompt setting` : prompt the user interactively for a specific basic setting
- `sudo systemd-firstboot --force` : force writing configuration even if the relevant files already exist
- `sudo systemd-firstboot --reset` : remove all existing files that are configured by `systemd-firstboot`
- `sudo systemd-firstboot --delete-root-password` : remove the password of the system's root user

## Categories
system, users, files

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemd-firstboot
