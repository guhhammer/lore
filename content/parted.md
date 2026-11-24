# parted

A partition manipulation program.

## Usage
- `sudo parted -l` : list partitions on all block devices
- `sudo parted /dev/sdX mklabel aix|amiga|bsd|dvh|gpt|loop|mac|msdos|pc98|sun` : create a new partition table of the specified label-type
- `sudo parted /dev/sdX -s mklabel gpt mkpart "boot_partition_name" 0% 500MiB mkpart "system_partition_name" 500MiB 100%` : create a new `gpt` partition table with a 500MiB boot partition and give the rest for the system partition (`--script` skips user intervention prompts)
- `sudo parted /dev/sdX set 1 boot on` : set a partition to have its boot flag turned on
- `sudo parted /dev/sdX` : start interactive mode with the specified disk selected
- `parted -h` : display help

## See also
- parted.interactive
- cfdisk
- partprobe

## Categories
danger, disks

## Links
- More information: https://www.gnu.org/software/parted/manual/parted.html#Invoking-Parted
- tldr page: https://tldr.inbrowser.app/pages/linux/parted
- Wikipedia: https://en.wikipedia.org/wiki/GNU_Parted
