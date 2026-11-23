# depmod

Generate `modules.dep` and map files for Linux kernel modules.

## Usage
- `depmod` : generate dependency and map files for the current kernel
- `depmod -o path/to/directory` : generate dependency and map files in a specified directory
- `depmod kernel_version` : generate dependency and map files for a specific kernel version
- `depmod kernel_version /path/to/module.ko` : examine a specific module instead of all modules
- `depmod -b path/to/root_directory` : use an alternative root filesystem directory
- `depmod -n` : write dependency information to `stdout` instead of writing to files

## See also
- modprobe

## Categories
system, files

## Links
- More information: https://manned.org/depmod
- tldr page: https://tldr.inbrowser.app/pages/linux/depmod
