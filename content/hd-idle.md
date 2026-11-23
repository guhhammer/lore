# hd-idle

Spin down external disks after a period of idle time.

## Usage
- `systemctl start hd-idle` : start a service to manage hard drive spin down. By default hard disks will spin down after 10 minutes of inactivity
- `hd-idle -t /dev/sdX` : spin down a disk immediately
- `hd-idle -i 0 -a /dev/sda -i 300 -a /dev/sdb -i 1200` : set disks to never spin down, then set explicit idle times (in seconds) for disks which have "sda" or "sdb" in their device name

## Categories
disks

## Links
- More information: https://manned.org/hd-idle
- tldr page: https://tldr.inbrowser.app/pages/linux/hd-idle
