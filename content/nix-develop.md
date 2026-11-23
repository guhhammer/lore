# nix develop

Run a Bash shell that provides the build environment of a derivation.

## Usage
- `nix develop nixpkgs#pkg` : start a shell with all dependencies of a package from nixpkgs available
- `nix develop` : start a development shell for the default package in a flake in the current directory
- `configurePhase; buildPhase` : in that shell, configure and build the sources

## Categories
shell, development, packages

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/new-cli/nix3-develop.html
- tldr page: https://tldr.inbrowser.app/pages/common/nix-develop
