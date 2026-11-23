# minicom

Communicate with the serial interface of a device.

## Usage
- `sudo minicom -D /dev/ttyXYZ` : open a given serial port
- `sudo minicom -D /dev/ttyXYZ -b 115200` : open a given serial port with a given baud rate
- `sudo minicom -D /dev/ttyXYZ -s` : enter the configuration menu before communicating with a given serial port
- `sudo minicom -D /dev/ttyXYZ -C path/to/file` : capture the output of a serial port to a file
- `<Ctrl a><x><Enter>` : exit minicom
- `minicom -h` : display help

## See also
- picocom
- cu
- tio

## Categories
network, users, files

## Links
- More information: https://manned.org/minicom
- tldr page: https://tldr.inbrowser.app/pages/linux/minicom
