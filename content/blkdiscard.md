# blkdiscard

Discard device sectors on storage devices. Useful for SSDs.

## Usage
- `blkdiscard /dev/device` : discard all sectors on a device, removing all data
- `blkdiscard -s /dev/device` : securely discard all blocks on a device, removing all data
- `blkdiscard -l 100MB /dev/device` : discard the first 100 MB of a device

## Categories
hardware

## Links
- More information: https://manned.org/blkdiscard
- tldr page: https://tldr.inbrowser.app/pages/linux/blkdiscard
