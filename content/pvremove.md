# pvremove

Remove LVM labels from physical volume(s).

## Usage
- `sudo pvremove /dev/sdXY` : remove an LVM label from a physical volume
- `sudo pvremove -v /dev/sdXY` : display detailed output during the operation
- `sudo pvremove -y /dev/sdXY` : remove an LVM label without asking for confirmation
- `sudo pvremove -f /dev/sdXY` : forcefully remove an LVM label
- `sudo pvremove --reportformat json /dev/sdXY` : display output in JSON format

## Categories
danger, disks, users

## Links
- More information: https://manned.org/pvremove
- tldr page: https://tldr.inbrowser.app/pages/linux/pvremove
