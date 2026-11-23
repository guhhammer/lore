# mkfs.erofs

Create an EROFS filesystem in an image.

## Usage
- `mkfs.erofs image.erofs root/` : create an EROFS filesystem based on the root directory
- `mkfs.erofs -U uuid image.erofs root/` : create an EROFS image with a specific UUID
- `mkfs.erofs -zlz4hc image.erofs root/` : create a compressed EROFS image
- `mkfs.erofs --all-root image.erofs root/` : create an EROFS image where all files are owned by root

## Categories
danger, containers, media, disks

## Links
- More information: https://manned.org/mkfs.erofs
- tldr page: https://tldr.inbrowser.app/pages/linux/mkfs.erofs
