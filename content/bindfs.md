# bindfs

Mount a directory elsewhere with different permissions.

## Usage
- `sudo bindfs path/to/directory path/to/mount_point` : mount a directory with same permissions
- `sudo bindfs --map=user1/user2 path/to/directory path/to/mount_point` : map filesystem objects owned by `user1` to be owned by `user2` (also applies in reverse to newly created files)
- `sudo umount path/to/mount_point` : unmount a directory

## Categories
files, disks, users

## Links
- More information: https://bindfs.org/docs/bindfs.1.html
- tldr page: https://tldr.inbrowser.app/pages/linux/bindfs
