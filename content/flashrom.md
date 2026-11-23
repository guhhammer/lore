# flashrom

Read, write, verify, and erase flash chips.

## Usage
- `flashrom -p programmer` : probe the chip, ensuring the wiring is correct
- `flashrom -p programmer -r path/to/file` : read flash and save it to a file
- `flashrom -p programmer -w path/to/file` : write a file to the flash
- `flashrom -p programmer -v path/to/file` : verify the flash against a file
- `flashrom -p linux_spi:dev=/dev/spidev0.0` : probe the chip using Raspberry Pi

## Categories
files

## Links
- More information: https://manned.org/flashrom
- tldr page: https://tldr.inbrowser.app/pages/linux/flashrom
