# minipro

Control Xgecu chip programmers (TL866A/CS, TL866II+, T48, T56). Supports various chips including AVRs, PICs, microcontrollers, and memory chips.

## Usage
- `minipro -l` : list all supported devices
- `minipro -L device_name` : search for a specific device
- `minipro -p chip_name -D` : read chip ID
- `minipro -p chip_name -r path/to/output_file.bin` : read chip contents to a file
- `minipro -p chip_name -w path/to/input_file.bin` : write a file to chip
- `minipro -p chip_name -m path/to/file.bin` : verify chip contents against a file
- `minipro -p chip_name -E` : erase a chip
- `minipro -h` : display help

## Categories
files, monitoring

## Links
- More information: https://manned.org/minipro
- tldr page: https://tldr.inbrowser.app/pages/common/minipro
