# udevadm

Linux `udev` management tool.

## Usage
- `sudo udevadm monitor` : monitor all device events
- `sudo udevadm monitor -k` : print `uevents` sent out by the kernel
- `sudo udevadm monitor -u` : print device events after being processed by `udev`
- `sudo udevadm info -a /dev/sda` : list attributes of device `/dev/sda`
- `sudo udevadm control -R` : reload all `udev` rules
- `sudo udevadm trigger` : trigger all `udev` rules to run
- `sudo udevadm test /dev/sda` : test an event run by simulating loading of `/dev/sda`

## Categories
users, monitoring, hardware

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/udevadm.html
- tldr page: https://tldr.inbrowser.app/pages/linux/udevadm
