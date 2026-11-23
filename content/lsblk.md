# lsblk

List information about devices.

## Usage
- `lsblk` : list all storage devices in a tree-like format
- `lsblk -a` : also list empty devices
- `lsblk -b` : print the SIZE column in bytes rather than in a human-readable format
- `lsblk -f` : output info about filesystems
- `lsblk -i` : use ASCII characters for tree formatting
- `lsblk -t` : output info about block-device topology
- `lsblk -e 1,7,...` : exclude the devices specified by the comma-separated list of major device numbers
- `lsblk -o +NAME,ROTA,SERIAL,MODEL,TRAN,TYPE,SIZE,FSTYPE,MOUNTPOINT,...` : add extra information to the output using a comma-separated list of columns (omit the `+` sign to only display the specified columns)

## Categories
disks, hardware

## Links
- More information: https://manned.org/lsblk
- tldr page: https://tldr.inbrowser.app/pages/linux/lsblk
