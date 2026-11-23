# mount.steamos

Mount or unmount SteamOS filesystem partitions.

## Usage
- `sudo mount.steamos /dev/sdX /mnt` : mount all necessary partitions from a device to a target directory
- `sudo mount.steamos -o nohome,nooverlay /dev/sdX /mnt` : mount with options to exclude specific partitions (e.g. `/home`, overlays)
- `sudo mount.steamos -u /mnt` : unmount all partitions mounted under a target directory
- `mount.steamos -h` : display help

## Categories
disks, users

## Links
- More information: https://gitlab.com/users/evlaV/projects
- tldr page: https://tldr.inbrowser.app/pages/linux/mount.steamos
