# mhwd

Manjaro Hardware Detection utility.

## Usage
- `mhwd -l` : list available drivers
- `mhwd -li` : list installed drivers
- `mhwd -i pci|usb driver_name` : install a driver
- `mhwd -r pci|usb driver_name` : remove a driver
- `mhwd -l -d` : display detailed information about detected hardware
- `mhwd -a pci free|nonfree 0300` : automatically install best available driver for the detected graphics card

## Categories
hardware

## Links
- More information: https://wiki.manjaro.org/index.php/Manjaro_Hardware_Detection_Overview
- tldr page: https://tldr.inbrowser.app/pages/linux/mhwd
