# nix-store.3

Manipulate the Nix store.

## Usage
- `nix store gc` : collect garbage, i.e. remove unused paths to reduce space usage
- `nix store optimise` : hard-link identical files together to reduce space usage
- `nix store delete /nix/store/checksum-package-version.ext` : delete a specific store path (must be unused)
- `nix store --store https://cache.nixos.org ls /nix/store/checksum-package-version.ext` : list a contents of the store path, on a remote store
- `nix store diff-closures /nix/store/checksum-package-version.ext /nix/store/checksum-package-version.ext` : show the differences in versions between two store paths, with their respective dependencies

## See also
- nix-store

## Categories
packages, security, files

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/new-cli/nix3-store.html
- tldr page: https://tldr.inbrowser.app/pages/common/nix-store.3
