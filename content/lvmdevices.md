# lvmdevices

Manage the LVM devices file that lists block devices allowed for physical volumes.

## Usage
- `sudo lvmdevices` : list devices recorded in the devices file
- `sudo lvmdevices --adddev /dev/sdXN` : add a device to the devices file
- `sudo lvmdevices --deldev /dev/sdXN` : remove a device from the devices file
- `sudo lvmdevices --addpvid pvid` : add a physical volume by its PVID
- `sudo lvmdevices --delpvid pvid` : remove a physical volume by its PVID
- `sudo lvmdevices --update` : update the devices file after device names change
- `sudo lvmdevices --check` : check the devices file for problems
- `lvmdevices --version` : display version

## Categories
hardware, files, users

## Links
- More information: https://manned.org/lvmdevices
- tldr page: https://tldr.inbrowser.app/pages/linux/lvmdevices
