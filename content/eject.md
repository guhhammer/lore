# eject

Eject CDs, floppy disks, tape drives, and USB sticks.

## Usage
- `eject -d` : display the default device
- `eject` : eject the default device
- `eject /dev/cdrom` : eject a specific device (the default order is cd-rom, scsi, floppy, and tape)
- `eject -T /dev/cdrom` : toggle whether a device's tray is open or closed
- `eject -r /dev/cdrom` : eject a cd drive
- `eject -f /mnt/floppy` : eject a floppy drive
- `eject -q /mnt/tape` : eject a tape drive
- `eject -i on|off` : set whether the physical eject button is ignored (`on` prevents ejecting)

## Categories
hardware, disks

## Links
- More information: https://manned.org/eject
- tldr page: https://tldr.inbrowser.app/pages/linux/eject
