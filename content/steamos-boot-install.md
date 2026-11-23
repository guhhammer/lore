# steamos-boot-install

This command is an alias of `steamos-finalize-install`.

## Usage
- `sudo steamos-finalize-install` : finalize the installation
- `sudo steamos-finalize-install --no-bootloaders --no-kernel` : finalize without updating bootloaders or kernel
- `sudo steamos-finalize-install --no-migrate` : skip all migration steps
- `sudo steamos-finalize-install --roothash hash` : set a specific root hash during finalization
- `sudo steamos-finalize-install --force` : force system migration steps regardless of environment

## Categories
packages, system, users

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/steamos-boot-install
