# nh

Modern helper utility tool for the Nix/NixOS ecosystem. Some subcommands such as `os`, `home`, `clean`, and `search` have their own usage documentation.

## Usage
- `nh os switch path/to/flake` : build and switch to a specified NixOS flake configuration
- `nh home switch path/to/flake` : build and switch to a specified Home Manager flake configuration
- `nh darwin switch path/to/flake -H host` : build and switch to a nix-darwin flake configuration
- `nh clean all -a` : collect all garbage and gcroots from the Nix store
- `nh search name` : search for a package in Nixpkgs
- `nh completions shell` : generate shell completions for a specified shell

## Categories
monitoring, development, documents

## Links
- More information: https://github.com/nix-community/nh#usage
- tldr page: https://tldr.inbrowser.app/pages/common/nh
