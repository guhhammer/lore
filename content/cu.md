# cu

Call Up another system and act as a dial-in/serial terminal or perform file transfers with no error checking.

## Usage
- `sudo cu -l /dev/ttyXYZ` : open a given serial port
- `sudo cu -l /dev/ttyXYZ -s 115200` : open a given serial port with a given baud rate
- `sudo cu -l /dev/ttyXYZ -s 115200 -h` : open a given serial port with a given baud rate and echo characters locally (half-duplex mode)
- `sudo cu -l /dev/ttyXYZ -s 115200 --parity=even|odd|none -f --nostop` : open a given serial port with a given baud rate, parity, and no hardware or software flow control
- `<Enter><~><.>` : exit the `cu` session when in connection
- `cu --help` : display help

## See also
- picocom
- minicom
- tio

## Categories
network, users, files

## Links
- More information: https://manned.org/cu
- tldr page: https://tldr.inbrowser.app/pages/linux/cu
