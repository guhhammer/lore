# ddrescue

Data recovery tool that reads data from damaged block devices.

## Usage
- `sudo ddrescue /dev/sdX path/to/image.img` : take an image of a device
- `sudo ddrescue /dev/sdX path/to/image.img path/to/log.txt` : take an image of a device, creating a log file
- `sudo ddrescue -f -n /dev/sdX /dev/sdY path/to/log.txt` : clone Disk A to Disk B, creating a log file

## Categories
files, hardware, containers

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/ddrescue
