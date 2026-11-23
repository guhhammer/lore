# pacstrap

Install Arch Linux packages in the specified new root directory.

## Usage
- `pacstrap path/to/new_root base linux linux-firmware` : install the `base` package, the Linux kernel, and firmware for common hardware
- `pacstrap path/to/new_root base base-devel linux-lts` : install the `base` package, the Linux LTS kernel, and `base-devel` build tools
- `pacstrap -P path/to/new_root packages` : install packages and copy the host's Pacman config to the target
- `pacstrap -M path/to/new_root packages` : install packages without copying the host's mirrorlist to the target
- `pacstrap -C path/to/pacman.conf path/to/new_root packages` : use an alternate configuration file for Pacman
- `pacstrap -c path/to/new_root packages` : install packages using the package cache on the host instead of on the target
- `pacstrap -K path/to/new_root packages` : initialize an empty `pacman` keyring in the target without copying it from the host
- `pacstrap -i path/to/new_root packages` : install packages in interactive mode (prompts for confirmation)

## Categories
packages, files, network

## Links
- More information: https://manned.org/pacstrap
- tldr page: https://tldr.inbrowser.app/pages/linux/pacstrap
