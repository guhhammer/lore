# nh home

Manage and configure per-user environment using Nix. Reimplementation of `home-manager`.

## Usage
- `nh home switch path/to/flake` : build and switch to a specified Home Manager flake configuration
- `nh home build path/to/flake -u` : update all the flake inputs of the specified Home Manager flake configuration and build it
- `nh home repl path/to/flake` : load a specified Home Manager flake configuration in Nix REPL (Nix evaluation environment)

## Categories
development, files, users

## Links
- More information: https://github.com/nix-community/nh#usage
- tldr page: https://tldr.inbrowser.app/pages/common/nh-home
