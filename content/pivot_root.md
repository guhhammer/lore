# pivot_root

Change the root filesystem to a new directory and move the current root to a subdirectory of the new root. Commonly used during system initialization (e.g., in `initramfs`) to switch from a temporary root to the real root filesystem.

## Usage
- `sudo pivot_root path/to/new_root path/to/new_root/old_root` : make `/new_root` the new root (`/`) and move current root to a subdirectory of it
- `pivot_root -h` : display help

## Categories
files, disks

## Links
- More information: https://manned.org/pivot_root.8
- tldr page: https://tldr.inbrowser.app/pages/linux/pivot_root
