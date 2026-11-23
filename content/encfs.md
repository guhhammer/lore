# encfs

Mount or create encrypted virtual filesystems.

## Usage
- `encfs /path/to/cipher_directory /path/to/mount_point` : initialize or mount an encrypted filesystem
- `encfs --standard /path/to/cipher_directory /path/to/mount_point` : initialize an encrypted filesystem with standard settings
- `encfs -f /path/to/cipher_directory /path/to/mount_point` : run encfs in the foreground instead of spawning a daemon
- `encfs --reverse path/to/plain_directory path/to/cipher_directory` : mount an encrypted snapshot of a plain directory

## See also
- fusermount

## Categories
files, disks

## Links
- More information: https://manned.org/encfs
- tldr page: https://tldr.inbrowser.app/pages/common/encfs
