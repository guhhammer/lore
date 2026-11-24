# bcachefs

Manage `bcachefs` filesystems/devices. Some subcommands such as `device` have their own usage documentation.

## Usage
- `sudo bcachefs format path/to/partition` : format a partition with `bcachefs`
- `sudo bcachefs mount path/to/partition path/to/mount_point` : mount a `bcachefs` filesystem
- `sudo bcachefs format -l ssd.ssd1 path/to/ssd_partition -l hdd.hdd1 path/to/hdd_partition --replicas 1 --foreground_target ssd --promote_target ssd --background_target hdd` : create a RAID 0 filesystem where an SSD acts as a cache and an HDD acts as a long-term storage
- `sudo bcachefs mount path/to/partition1:path/to/partition2 path/to/mount_point` : mount a multidevice filesystem
- `bcachefs fs usage -h path/to/mount_point` : display disk usage
- `sudo bcachefs set-fs-option --metadata_replicas 2 --data_replicas 2 path/to/partition` : set replicas after formatting and mounting
- `sudo bcachefs data rereplicate path/to/mount_point` : force `bcachefs` to ensure all files are replicated
- `bcachefs subvolume snapshot path/to/directory path/to/snapshot` : create a snapshot of a particular directory

## Categories
files, disks, hardware

## Links
- More information: https://bcachefs-docs.readthedocs.io/en/latest/index.html
- tldr page: https://tldr.inbrowser.app/pages/linux/bcachefs
- Wikipedia: https://en.wikipedia.org/wiki/Bcachefs
