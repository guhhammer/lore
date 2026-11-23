# chmem

Modify the state of memory blocks (online or offline) in a Linux system. Typically used in virtualized environments to manage memory hotplug.

## Usage
- `sudo chmem -b -d block_number` : set a memory block offline
- `sudo chmem -b -e block_number` : set a memory block online
- `sudo chmem -d 0xstart_address-0xend_address` : set a memory range offline using hexadecimal addresses
- `sudo chmem -e 0xstart_address-0xend_address` : set a memory range online using hexadecimal addresses
- `sudo chmem -e 0xstart_address -z Movable` : set memory online and assign it to a specific zone (e.g., Movable)
- `chmem -h` : display help

## Categories
monitoring, users

## Links
- More information: https://manned.org/chmem
- tldr page: https://tldr.inbrowser.app/pages/linux/chmem
