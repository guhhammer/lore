# fwupdmgr

Update device firmware, including UEFI, using `fwupd`.

## Usage
- `fwupdmgr get-devices` : display all devices detected by `fwupd`
- `fwupdmgr refresh` : download the latest firmware metadata from LVFS
- `fwupdmgr get-updates` : list the updates available for devices on your system
- `fwupdmgr update` : install firmware updates
- `sudo mount -o uid=1000,gid=1000,umask=0022 /dev/sdX /boot` : remount `/boot` with more privileges if update complains about a read-only filesystem
- `fwupdmgr get-history` : show firmware update history

## See also
- fwupdtool

## Categories
hardware, network

## Links
- More information: https://github.com/fwupd/fwupd/blob/main/src/fwupdmgr.md
- tldr page: https://tldr.inbrowser.app/pages/linux/fwupdmgr
