# gdd

This command is an alias of GNU `dd`.

## Usage
- `dd if=path/to/file.iso of=/dev/usb_drive status=progress` : make a bootable USB drive from an isohybrid file (such as `archlinux-xxx.iso`) and show the progress
- `dd bs=4194304 conv=fsync if=/dev/source_drive of=/dev/dest_drive` : clone a drive to another drive with 4 MiB block size and flush writes before the command terminates
- `dd bs=100 count=1 if=/dev/urandom of=path/to/random_file` : generate a file with a specific number of random bytes by using kernel random driver
- `dd bs=1024 count=1000000 if=/dev/zero of=path/to/file_1GB` : benchmark the sequential write performance of a disk
- `dd if=/dev/drive_device of=path/to/file.img status=progress` : create a system backup, save it into an IMG file (can be restored later by swapping `if` and `of`), and show the progress

## Categories
files, disks, shell, macos

## Links
- tldr page: https://tldr.inbrowser.app/pages/osx/gdd
