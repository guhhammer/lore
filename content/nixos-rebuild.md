# nixos-rebuild

Reconfigure a NixOS machine.

## Usage
- `sudo nixos-rebuild switch` : build and switch to the new configuration, making it the boot default
- `sudo nixos-rebuild switch -p name` : build and switch to the new configuration, making it the boot default and naming the boot entry
- `sudo nixos-rebuild switch --upgrade` : build and switch to the new configuration, making it the boot default and installing updates
- `sudo nixos-rebuild switch --rollback` : roll back changes to the configuration, switching to the previous generation
- `sudo nixos-rebuild boot` : build the new configuration and make it the boot default without switching to it
- `sudo nixos-rebuild test` : build and activate the new configuration, but don't make a boot entry (for testing purposes)
- `sudo nixos-rebuild build-vm` : build the configuration and open it in a virtual machine
- `nixos-rebuild list-generations` : list available generations similar to the boot loader menu

## Categories
development, system, users

## Links
- More information: https://nixos.org/nixos/manual/#sec-changing-config
- tldr page: https://tldr.inbrowser.app/pages/linux/nixos-rebuild
