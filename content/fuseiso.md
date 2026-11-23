# fuseiso

FUSE module to mount ISO, BIN, NRG, MDF and IMG filesystem images.

## Usage
- `fuseiso path/to/image.iso path/to/mount_point` : mount an ISO image to a given directory
- `fuseiso -p path/to/image.iso path/to/mount_point` : mount an ISO image, automatically creating the mount point if it doesn't exist (and deleting it on unmount)
- `fuseiso -f path/to/image.iso path/to/mount_point` : mount an ISO image in the foreground (useful for debugging)
- `fusermount -u path/to/mount_point` : unmount an image using the standard FUSE unmount utility

## Categories
containers, disks, media

## Links
- More information: https://manned.org/fuseiso
- tldr page: https://tldr.inbrowser.app/pages/linux/fuseiso
