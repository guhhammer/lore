# limine

An advanced, portable, multiprotocol bootloader.

## Usage
- `limine bios-install /dev/sdX` : install Limine to an MBR-partitioned disk
- `limine bios-install /dev/sdX partition_number` : install Limine to a GPT-partitioned disk with a specific BIOS boot partition
- `limine bios-install path/to/image.iso` : install Limine to a disk image file
- `limine-install` : install Limine for modern UEFI systems (requires the `limine-entry-tool` package)
- `limine-update` : update boot entries after a kernel upgrade (requires the `limine-entry-tool` package)
- `limine-scan` : scan for other operating systems to add to the boot menu (requires the `limine-entry-tool` package)

## See also
- limine-enroll-config
- limine-entry-tool
- limine-snapper-sync

## Categories
packages, disks, hardware

## Links
- More information: https://codeberg.org/Limine/Limine
- tldr page: https://tldr.inbrowser.app/pages/linux/limine
