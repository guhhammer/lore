# nix-build.2

Construye una expresión de Nix.

## Usage
- `nix-build '<nixpkgs>' -A firefox` : construye una expresión de Nix
- `nix-build '<nixpkgs>' -A firefox --option sandbox true` : construye una expresión de Nix aislada (en sistemas que no son NixOS)

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/nix-build.html
