# home-manager

Manage a per-user environment using Nix, allowing declarative configuration of the user's home.

## Usage
- `home-manager build` : build the configuration defined in `~/.config/nixpkgs/home.nix` without applying it
- `home-manager switch` : build and apply (switch to) the new configuration
- `home-manager rollback` : roll back to a previous configuration generation
- `home-manager generations` : list all existing configuration generations
- `home-manager command --flake path/to/flake` : when using flakes, run any operation that requires nix to run (build, switch, news) by passing the path to the flake

## Categories
users, development

## Links
- More information: https://github.com/nix-community/home-manager
- tldr page: https://tldr.inbrowser.app/pages/common/home-manager
