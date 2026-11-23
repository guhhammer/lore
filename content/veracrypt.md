# veracrypt

Free and open source disk encryption software.

## Usage
- `veracrypt -t -c --random-source=/dev/urandom` : create a new volume through a text user interface and use `/dev/urandom` as a source of random data
- `veracrypt -t path/to/volume path/to/mount_point` : decrypt a volume interactively through a text user interface and mount it to a directory
- `veracrypt -k path/to/keyfile /dev/sdXN path/to/mount_point` : decrypt a partition using a keyfile and mount it to a directory
- `veracrypt -d path/to/mounted_point` : dismount a volume on the directory it is mounted to

## Categories
disks, files, network

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/veracrypt
