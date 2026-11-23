# nh os

Reconfigure or debug a NixOS machine.

## Usage
- `nh os switch path/to/flake` : build and switch to a specified NixOS flake configuration
- `nh os boot path/to/flake -u` : update all the flake inputs of the specified NixOS flake configuration, build it, and make it the boot default
- `nh os test path/to/flake -s specialisation` : build and activate a specified NixOS flake configuration specialisation
- `nh os build-vm path/to/flake -H host` : build a specified NixOS flake configuration host and create a symlink of the result from the Nix store in the current directory
- `nh os repl path/to/flake` : load a specified NixOS flake configuration in Nix REPL (Nix evaluation environment)
- `nh os info` : list all available generations from profile path
- `nh os rollback -t generation` : roll back to a specified generation

## Categories
development, files, network

## Links
- More information: https://github.com/nix-community/nh#usage
- tldr page: https://tldr.inbrowser.app/pages/common/nh-os
