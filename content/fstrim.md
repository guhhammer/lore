# fstrim

Discard unused blocks on a mounted filesystem. Only supported by flash memory devices such as SSDs and microSD cards.

## Usage
- `sudo fstrim -a` : trim unused blocks on all mounted partitions that support it
- `sudo fstrim /` : trim unused blocks on a specified partition
- `sudo fstrim -v /` : display statistics after trimming

## Categories
disks, monitoring, hardware

## Links
- More information: https://manned.org/fstrim
- tldr page: https://tldr.inbrowser.app/pages/linux/fstrim
