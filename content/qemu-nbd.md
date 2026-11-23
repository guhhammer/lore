# qemu-nbd

QEMU Disk Network Block Device Server.

## Usage
- `qemu-nbd [OPTION]... filename` : the general form, as the manual gives it
- `qemu-nbd -L [OPTION]...` : another form of the same command
- `qemu-nbd -d dev` : another form of the same command
- `qemu-nbd -o` : the offset into the image
- `qemu-nbd -k` : use a unix socket with path PATH
- `qemu-nbd -r` : export the disk as read-only
- `qemu-nbd -n` : equivalent to --cache=none

## Categories
disks, network, hardware
