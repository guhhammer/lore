# btrfs send

Transfer data (subvolume/snapshot) from one filesystem to another in a streamable format. Note: `send` requires the snapshot be read-only.

## Usage
- `sudo btrfs send -f path/to/file.btrfs path/to/source_snapshot` : send a subvolume snapshot to a file instead of `stdout`
- `sudo btrfs receive -f path/to/file.btrfs path/to/destination_directory` : restore a subvolume snapshot from a file instead of `stdin`
- `sudo btrfs send path/to/source_snapshot | sudo btrfs receive path/to/destination_directory` : send a subvolume snapshot stream and pipe it to a receiving destination
- `sudo btrfs send -p path/to/parent_snapshot -f path/to/incremental.btrfs path/to/new_snapshot` : send an incremental update (differences between parent and new snapshot) to a file
- `sudo btrfs send -p path/to/parent_snapshot path/to/new_snapshot | sudo btrfs receive path/to/destination_directory` : send an incremental update stream and pipe it to a receiving destination
- `sudo btrfs send path/to/source_snapshot | ssh user@host "sudo btrfs receive path/to/destination_directory"` : send a subvolume snapshot over SSH (the receiving `sudo` must be configured for passwordless execution)
- `sudo btrfs receive --dump -f path/to/file.btrfs` : dump stream metadata from a file without modifying the filesystem

## See also
- btrfs-subvolume
- btrfs-property

## Categories
files, users, network

## Links
- More information: https://btrfs.readthedocs.io/en/latest/Send-receive.html
- tldr page: https://tldr.inbrowser.app/pages/linux/btrfs-send
