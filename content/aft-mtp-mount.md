# aft-mtp-mount

Mount MTP (Media Transfer Protocol) devices to the local filesystem using FUSE.

## Usage
- `aft-mtp-mount path/to/mount_point` : mount an MTP device to a directory
- `aft-mtp-mount -D device_name path/to/mount_point` : mount a specific device to a directory
- `aft-mtp-mount -R path/to/mount_point` : reset the device before mounting
- `aft-mtp-mount -C path/to/mount_point` : mount a device without claiming the USB interface
- `aft-mtp-mount -d path/to/mount_point` : mount a device and display debug output
- `aft-mtp-mount -h` : display help

## See also
- fusermount

## Categories
disks, hardware, files

## Links
- More information: https://github.com/whoozle/android-file-transfer-linux
- tldr page: https://tldr.inbrowser.app/pages/common/aft-mtp-mount
