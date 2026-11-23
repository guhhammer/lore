# steamos-finalize-install

Complete a SteamOS installation by setting up bootloaders and applying system updates.

## Usage
- `sudo steamos-finalize-install` : finalize the installation
- `sudo steamos-finalize-install --no-bootloaders --no-kernel` : finalize without updating bootloaders or kernel
- `sudo steamos-finalize-install --no-migrate` : skip all migration steps
- `sudo steamos-finalize-install --roothash hash` : set a specific root hash during finalization
- `sudo steamos-finalize-install --force` : force system migration steps regardless of environment

## Categories
packages, users

## Links
- More information: https://gitlab.com/users/evlaV/projects
- tldr page: https://tldr.inbrowser.app/pages/linux/steamos-finalize-install
