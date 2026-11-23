# nix-build

`nix-build` can refer to multiple commands with the same name.

## Usage
- `nix-build '<nixpkgs>' -A firefox` : build a Nix expression
- `nix-build '<nixpkgs>' -A firefox --option sandbox true` : build a sandboxed Nix expression (on non-NixOS)

## Categories
development

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/nix-build
