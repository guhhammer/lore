# akmods

Automatic Kernel Module management system for Fedora/RHEL-based distributions. Rebuilds out-of-tree kernel modules (like NVIDIA drivers) dynamically when kernels update.

## Usage
- `sudo akmods` : rebuild all missing or outdated kernel modules for the currently running kernel
- `sudo akmods --rebuild --force` : forcefully rebuild all kernel modules regardless of current status
- `sudo akmods --kernel kernel_version` : rebuild modules specifically for a targeted kernel version
- `systemctl status akmods` : display the current status and log details of the background akmods service

## Categories
system, development, users

## Links
- More information: https://manned.org/akmods
- tldr page: https://tldr.inbrowser.app/pages/linux/akmods
