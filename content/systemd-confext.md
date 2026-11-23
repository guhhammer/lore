# systemd-confext

This command is an alias of `systemd-sysext`. It follows the same principle as `systemd-sysext`, but instead of working on `/usr` and `/opt`, `confext` will extend only `/etc`.

## Usage
- `systemd-sysext list` : list installed extension images
- `systemd-sysext merge` : merge system extension images into `/usr/` and `/opt/`
- `systemd-sysext` : check the current merge status
- `systemd-sysext unmerge` : unmerge all currently installed system extension images from `/usr/` and `/opt/`
- `systemd-sysext refresh` : refresh system extension images (a combination of `unmerge` and `merge`)

## Categories
system, containers, media

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemd-confext
