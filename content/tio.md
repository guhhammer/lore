# tio

Monitor and interact with serial ports.

## Usage
- `tio /dev/ttyUSB0` : open a serial port with default settings
- `tio -b 9600 /dev/ttyUSB0` : open a serial port with a specific baud rate
- `tio -L --log-file log_file /dev/ttyUSB0` : open a serial port and log output to a file
- `tio --output-mode hex /dev/ttyUSB0` : open a serial port and enable hexadecimal output
- `tio -l` : list available serial ports
- `<Ctrl t><q>` : exit `tio` session

## See also
- picocom
- cu
- minicom

## Categories
monitoring, network

## Links
- More information: https://github.com/tio/tio#3-usage
- tldr page: https://tldr.inbrowser.app/pages/common/tio
