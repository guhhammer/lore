# parted.interactive

A partition manipulation program.

## Usage
- `sudo parted /dev/sdX` : start interactive mode with the specified disk selected
- `print` : [Interactive] Show partition information in interactive mode
- `select /dev/sdX` : [Interactive] Select a disk in interactive mode
- `mkpart partition_name btrfs|ext2|ext3|ext4|fat16|fat32|hfs|hfs+|linux-swap|ntfs|reiserfs|udf|xfs 0% 16G` : [Interactive] Create a 16 GB partition with the specified filesystem in interactive mode (`GPT` partition table)
- `mkpart primary|logical|extended btrfs|ext2|ext3|ext4|fat16|fat32|hfs|hfs+|linux-swap|ntfs|reiserfs|udf|xfs 0% 16G` : [Interactive] Create a 16 GB partition with the specified filesystem in interactive mode (`MBR` partition table)
- `resizepart /dev/sdXN end_position_of_partition` : [Interactive] Resize a partition in interactive mode
- `rm /dev/sdXN` : [Interactive] Remove a partition in interactive mode
- `?` : [Interactive] Display help

## See also
- partprobe

## Categories
disks

## Links
- More information: https://www.gnu.org/software/parted/parted.html
- tldr page: https://tldr.inbrowser.app/pages/linux/parted.interactive
