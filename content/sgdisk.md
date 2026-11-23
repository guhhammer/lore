# sgdisk

Manage GUID Partition Tables (GPT). Part of the GPT fdisk suite; designed for scripting and automation.

## Usage
- `sudo sgdisk -p /dev/sdX` : display basic GPT partition summary data for a device
- `sudo sgdisk -Z /dev/sdX` : wipe both GPT and MBR data structures from a device (destroys all partition information)
- `sudo sgdisk -m 1:2:3:4 /dev/sdX` : convert a GPT disk to MBR format using up to four partitions
- `sudo sgdisk -d 1 /dev/sdX` : delete a partition entry by number (data in sectors remains untouched)
- `sudo sgdisk -b /path/to/backup.gpt /dev/sdX` : save the current in-memory GPT data (protective MBR, headers, and table) to a binary backup file
- `sudo sgdisk -l /path/to/backup.gpt /dev/sdX` : load GPT data from a backup file (restoring from a non-original disk is not recommended)
- `sudo sgdisk -v /dev/sdX` : verify GPT structures for CRC errors, mismatches, or inconsistencies
- `sgdisk -L` : display a summary of available partition type codes (no device required)

## Categories
danger, disks, users, files

## Links
- More information: https://manned.org/sgdisk
- tldr page: https://tldr.inbrowser.app/pages/common/sgdisk
