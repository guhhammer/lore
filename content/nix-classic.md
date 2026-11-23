# nix classic

A classic, stable interface to a powerful package manager that makes package management reliable, reproducible, and declarative. Some Nix commands such as `nix-build`, `nix-shell`, `nix-env`, and `nix-store` have their own pages.

## Usage
- `nix-env -qaP search_term_regex` : search for a package in nixpkgs via its name
- `nix-shell -p pkg1 pkg2 pkg3 ...` : start a shell with the specified packages available
- `nix-env -iA nixpkgs.pkg1 nixpkgs.pkg2 ...` : install some packages permanently
- `nix-store -q --tree /nix/store/checksum-package-version.ext` : show all dependencies of a store path (package), in a tree format
- `nix-channel --update` : update the channels (repositories)
- `nix-collect-garbage` : remove unused paths from Nix store

## Categories
packages, network, shell

## Links
- More information: https://nixos.org/
- tldr page: https://tldr.inbrowser.app/pages/common/nix-classic
