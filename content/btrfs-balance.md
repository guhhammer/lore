# btrfs balance

Balance block groups on a btrfs filesystem.

## Usage
- `sudo btrfs b status path/to/btrfs_filesystem` : show the status of a running or paused balance operation
- `sudo btrfs b start path/to/btrfs_filesystem` : balance all block groups (slow; rewrites all blocks in filesystem)
- `sudo btrfs b start --bg -dusage=15 path/to/btrfs_filesystem` : balance data block groups which are less than 15% utilized, running the operation in the background
- `sudo btrfs b start -musage=20,limit=10,devid=devid path/to/btrfs_filesystem` : balance a max of 10 metadata chunks with less than 20% utilization and at least 1 chunk on a given device `devid` (see `btrfs filesystem show`)
- `sudo btrfs b start -dconvert=raid6 -mconvert=raid1c3 path/to/btrfs_filesystem` : convert data blocks to the raid6 and metadata to raid1c3 (see mkfs.btrfs(8) for profiles)
- `sudo btrfs b start -dconvert=raid1,soft path/to/btrfs_filesystem` : convert data blocks to raid1, skipping already converted chunks (e.g. after a previous cancelled conversion operation)
- `sudo btrfs b cancel|pause|resume path/to/btrfs_filesystem` : cancel, pause, or resume a running or paused balance operation

## Categories
users, files, disks

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-balance.html
- tldr page: https://tldr.inbrowser.app/pages/linux/btrfs-balance
