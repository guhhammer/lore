# nethogs

Monitor bandwidth usage per process.

## Usage
- `sudo nethogs` : start NetHogs as root (default device is `eth0`)
- `sudo nethogs device` : monitor bandwidth on specific device
- `sudo nethogs device1 device2 ...` : monitor bandwidth on multiple devices
- `sudo nethogs -d seconds` : specify refresh interval in seconds
- `sudo nethogs -v 0..5` : change the traffic display unit (default: 0, 0 = kB/s, 1 = total kB, 2 = total bytes, 3 = total MB, 4 = MB/s, 5 = GB/s)

## Categories
monitoring, users, hardware

## Links
- More information: https://manned.org/nethogs
- tldr page: https://tldr.inbrowser.app/pages/linux/nethogs
