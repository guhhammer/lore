# swapoff

Disable devices and files for swapping. Note: `path/to/file` can either point to a regular file or a swap partition.

## Usage
- `sudo swapoff path/to/file` : disable a given swap area
- `sudo swapoff -a` : disable all swap areas in `/proc/swaps`
- `sudo swapoff -L label` : disable a swap partition by its label

## Categories
files, disks, hardware

## Links
- More information: https://manned.org/swapoff.8
- tldr page: https://tldr.inbrowser.app/pages/linux/swapoff
