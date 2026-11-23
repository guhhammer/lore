# grub-probe

Probe device information for a particular path or device.

## Usage
- `sudo grub-probe -t fs /boot/grub` : get GRUB filesystem module for a path
- `sudo grub-probe -t device /boot/grub` : get the system device containing a path
- `sudo grub-probe -t drive /dev/sdX -d` : get GRUB disk name for a system device
- `sudo grub-probe -t fs_uuid /boot/grub` : get filesystem UUID
- `sudo grub-probe -t fs_label /boot/grub` : get filesystem label
- `sudo grub-probe -t msdos_parttype /dev/sdX` : get MBR partition type code(two hexadecimal digits)
- `sudo grub-probe -t drive /boot/grub -m path/to/custom_device.map` : probe using a custom device map

## Categories
hardware, disks, users

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/grub-probe
