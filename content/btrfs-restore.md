# btrfs restore

Try to salvage files from a damaged btrfs filesystem.

## Usage
- `sudo btrfs rest path/to/btrfs_device path/to/target_directory` : restore all files from a btrfs filesystem to a given directory
- `sudo btrfs rest -D path/to/btrfs_device path/to/target_directory` : simulate a restore, listing the files that would be restored from a btrfs filesystem
- `sudo btrfs rest --path-regex regex -c path/to/btrfs_device path/to/target_directory` : restore files matching a given `regex` (case-insensitive) files to be restored from a btrfs filesystem (all parent directories of target file(s) must match as well)
- `sudo btrfs rest -t bytenr path/to/btrfs_device path/to/target_directory` : restore files from a btrfs filesystem using a specific root tree `bytenr` (see `btrfs-find-root`)
- `sudo btrfs rest -m -x -S -o path/to/btrfs_device path/to/target_directory` : restore files from a btrfs filesystem (along with metadata, extended attributes, and Symlinks), overwriting files in the target

## Categories
files, disks, users

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-restore.html
- tldr page: https://tldr.inbrowser.app/pages/linux/btrfs-restore
- Wikipedia: https://en.wikipedia.org/wiki/Btrfs
