# f3probe

Probe a block device (e.g. a flash drive or a microSD card) for counterfeit flash memory.

## Usage
- `sudo f3probe path/to/block_device` : probe a block device
- `sudo f3probe -n path/to/block_device` : probe a whole block device destructively, without preserving any data (required for raw devices with no partition table)
- `sudo f3probe -l path/to/block_device` : use the minimum amount of RAM possible
- `sudo f3probe -t path/to/block_device` : time disk operations

## See also
- f3read
- f3write
- f3fix

## Categories
disks, hardware, files

## Links
- More information: https://manned.org/f3probe
- tldr page: https://tldr.inbrowser.app/pages/common/f3probe
