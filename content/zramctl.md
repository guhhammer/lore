# zramctl

Set up and control zram devices. Use `mkfs` or `mkswap` to format zram devices to partitions.

## Usage
- `lsmod | grep -i zram` : check if zram is enabled
- `sudo modprobe zram` : enable zram with a dynamic number of devices (use `zramctl` to configure devices further)
- `sudo modprobe zram num_devices=2` : enable zram with exactly 2 devices
- `sudo zramctl -f -s 2GB -a lz4` : find and initialize the next free zram device to a 2 GB virtual drive using LZ4 compression
- `sudo zramctl` : list currently initialized devices

## Categories
hardware, users

## Links
- More information: https://manned.org/zramctl
- tldr page: https://tldr.inbrowser.app/pages/linux/zramctl
