# nix registry

Manage a Nix flake registry.

## Usage
- `nix registry pin nixpkgs` : pin the `nixpkgs` revision to the current version of the upstream repository
- `nix registry pin entry github:owner/repo/branch_or_revision` : pin an entry to the latest version of the branch, or a particular revision of a GitHub repository
- `nix registry add entry github:owner/repo` : add a new entry that always points to the latest version of a GitHub repository, updating automatically
- `nix registry remove entry` : remove a registry entry
- `nix registry --help` : see documentation about what Nix flake registries are

## See also
- nix flake

## Categories
packages

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/new-cli/nix3-registry.html
- tldr page: https://tldr.inbrowser.app/pages/common/nix-registry
