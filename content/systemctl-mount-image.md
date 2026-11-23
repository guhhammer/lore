# systemctl mount-image

Mount an image file into a unit's mount namespace. Only supported for the units that run within a mountspace i.e. with `RootImage=`, `PrivateMounts=`, etc.

## Usage
- `systemctl mount-image unit /path/to/image /path/to/directory_inside_unit` : mount an image at a specific path inside the unit's mount namespace
- `systemctl mount-image unit /path/to/image /path/to/directory_inside_unit root:ro,nosuid` : mount the image's `root` partition with read-only and no-setuid options
- `systemctl mount-image --mkdir unit /path/to/image /path/to/directory_inside_unit` : create the destination directory before mounting
- `systemctl mount-image --read-only unit /path/to/image /path/to/directory_inside_unit` : mount an image as read-only

## Categories
disks, containers, media

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-mount-image
