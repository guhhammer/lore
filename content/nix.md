# nix

A powerful package manager that makes package management reliable, reproducible, and declarative. Some features of `nix` (`nix command`, `flakes`, etc.) are experimental and require enabling experimental features. Some subcommands such as `build`, `develop`, `flake`, `registry`, `profile`, `search`, `repl`, `store`, `edit`, `why-depends`, etc. have their own usage documentation.

## Usage
- `mkdir -p ~/.config/nix; echo 'experimental-features = nix-command flakes' >~/.config/nix/nix.conf` : enable the `nix` command
- `nix search nixpkgs search_term` : search for a package in nixpkgs via its name or description
- `nix shell nixpkgs#pkg1 nixpkgs#pkg2 nixpkgs#pkg3 ...` : start a shell with the specified packages from nixpkgs available
- `nix profile install nixpkgs#pkg1 nixpkgs#pkg2 nixpkgs#pkg3 ...` : install some packages from nixpkgs permanently
- `nix store gc` : remove unused paths from Nix store to free up space
- `nix repl` : start an interactive environment for evaluating Nix expressions
- `nix help subcommand` : display help for a specific subcommand

## See also
- nix classic

## Categories
packages, development, documents

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/new-cli/nix
- tldr page: https://tldr.inbrowser.app/pages/common/nix
