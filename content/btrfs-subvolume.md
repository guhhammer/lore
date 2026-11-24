# btrfs subvolume

Manage btrfs subvolumes and snapshots.

## Usage
- `sudo btrfs su c path/to/new_subvolume` : create a new empty subvolume
- `sudo btrfs su l path/to/btrfs_filesystem` : list all subvolumes and snapshots in the specified filesystem
- `sudo btrfs su d path/to/subvolume` : delete a subvolume
- `sudo btrfs su sn -r path/to/source_subvolume path/to/target` : create a read-only snapshot of an existing subvolume
- `sudo btrfs su sn path/to/source_subvolume path/to/target` : create a read-write snapshot of an existing subvolume
- `sudo btrfs su sh path/to/subvolume` : show detailed information about a subvolume

## Categories
files, users

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-subvolume.html
- tldr page: https://tldr.inbrowser.app/pages/linux/btrfs-subvolume
- Wikipedia: https://en.wikipedia.org/wiki/Btrfs
