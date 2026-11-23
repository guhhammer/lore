# growpart

Extend a partition in a disk or disk image to fill available space.

## Usage
- `growpart /dev/sdX n` : extend partition `n` from `sdX` to fill empty space until end of disk or beginning of next partition
- `growpart -N /path/to/disk.img n` : simulate growing partition `n` in a disk image, showing what modifications would be made

## Categories
disks, containers, media

## Links
- More information: https://github.com/canonical/cloud-utils
- tldr page: https://tldr.inbrowser.app/pages/linux/growpart
