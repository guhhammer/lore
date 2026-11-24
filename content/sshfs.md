# sshfs

Filesystem client based on SSH.

## Usage
- `sshfs username@remote_host:path/to/remote_directory path/to/mount_point` : mount remote directory
- `umount path/to/mount_point` : unmount remote directory
- `sshfs username@remote_host:path/to/remote_directory -p 2222 path/to/mount_point` : mount remote directory from server with specific port
- `sshfs username@remote_host:path/to/remote_directory -C path/to/mount_point` : use compression
- `sshfs -o follow_symlinks username@remote_host:path/to/remote_directory path/to/mount_point` : follow symbolic links

## Categories
files, network, disks

## Links
- More information: https://github.com/libfuse/sshfs/blob/master/sshfs.rst
- tldr page: https://tldr.inbrowser.app/pages/common/sshfs
- Wikipedia: https://en.wikipedia.org/wiki/SSHFS
