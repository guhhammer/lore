# nixos-option

Inspect a NixOS configuration.

## Usage
- `nixos-option option_key` : list all subkeys of a given option key
- `nixos-option boot.kernelModules` : list current boot kernel modules
- `nixos-option users.users.username.openssh.authorizedKeys.keyFiles|keys` : list authorized keys for a specific user
- `nixos-option nix.buildMachines` : list all remote builders
- `NIXOS_CONFIG=path/to/configuration.nix nixos-option option_key` : list all subkeys of a given key on another NixOS configuration
- `nixos-option -r users.users.user` : show recursively all values of a user

## Categories
users, system

## Links
- More information: https://nixos.org/manual/nixos/stable/index.html#sec-modularity
- tldr page: https://tldr.inbrowser.app/pages/linux/nixos-option
