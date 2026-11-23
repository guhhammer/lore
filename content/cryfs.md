# cryfs

A cryptographic filesystem for the cloud.

## Usage
- `cryfs path/to/cipher_directory path/to/mount_point` : mount an encrypted filesystem. The initialization wizard will be started on the first execution
- `cryfs-unmount path/to/mount_point` : unmount an encrypted filesystem
- `cryfs --unmount-idle 10 path/to/cipher_directory path/to/mount_point` : automatically unmount after ten minutes of inactivity
- `cryfs --show-ciphers` : list supported ciphers

## Categories
disks, files, cloud

## Links
- More information: https://manned.org/cryfs
- tldr page: https://tldr.inbrowser.app/pages/common/cryfs
