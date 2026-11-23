# systemd-sysext

Activate or deactivate system extension images.

## Usage
- `systemd-sysext list` : list installed extension images
- `systemd-sysext merge` : merge system extension images into `/usr/` and `/opt/`
- `systemd-sysext` : check the current merge status
- `systemd-sysext unmerge` : unmerge all currently installed system extension images from `/usr/` and `/opt/`
- `systemd-sysext refresh` : refresh system extension images (a combination of `unmerge` and `merge`)

## Categories
system, containers, media

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemd-sysext.html
- tldr page: https://tldr.inbrowser.app/pages/linux/systemd-sysext
