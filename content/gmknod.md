# gmknod

This command is an alias of GNU `mknod`.

## Usage
- `sudo mknod path/to/device_file b major_device_number minor_device_number` : create a block device
- `sudo mknod path/to/device_file c major_device_number minor_device_number` : create a character device
- `sudo mknod path/to/device_file p` : create a FIFO (queue) device
- `sudo mknod -Z path/to/device_file type major_device_number minor_device_number` : create a device file with default SELinux security context

## Categories
files, hardware, users, macos

## Links
- tldr page: https://tldr.inbrowser.app/pages/osx/gmknod
