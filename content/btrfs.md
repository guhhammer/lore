# btrfs

A filesystem based on the copy-on-write (COW) principle for Linux. Some subcommands such as `device` have their own usage documentation.

## Usage
- `sudo btrfs su c path/to/subvolume` : create subvolume
- `sudo btrfs su l path/to/mount_point` : list subvolumes
- `sudo btrfs f df path/to/mount_point` : show space usage information
- `sudo btrfs qu e path/to/subvolume` : enable quota
- `sudo btrfs qg s path/to/subvolume` : show quota

## Categories
files, users, monitoring

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs.html
- tldr page: https://tldr.inbrowser.app/pages/linux/btrfs
- Wikipedia: https://en.wikipedia.org/wiki/Btrfs
