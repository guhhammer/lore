# update-initramfs

Manage initramfs.

## Usage
- `sudo update-initramfs -c -k kernel_version` : create a new initramfs (use `all` for all installed kernel versions)
- `sudo update-initramfs -u` : update an existing initramfs
- `sudo update-initramfs -d -k kernel_version` : remove an existing initramfs (be careful when using `all` for `kernel_version`)

## Categories
users

## Links
- More information: https://manned.org/update-initramfs
- tldr page: https://tldr.inbrowser.app/pages/linux/update-initramfs
