# nix-store.2

Manipulate or query the Nix store.

## Usage
- `nix-store --gc` : collect garbage, such as removing unused paths
- `nix-store --optimise` : hard-link identical files together to reduce space usage
- `nix-store --delete /nix/store/checksum-package-version.ext` : delete a specific store path (must be unused)
- `nix-store -q --tree /nix/store/checksum-package-version.ext` : show all dependencies of a store path (package), in a tree format
- `du -cLsh $(nix-store -q --references /nix/store/checksum-package-version.ext)` : calculate the total size of a certain store path with all the dependencies
- `nix-store -q --referrers /nix/store/checksum-package-version.ext` : show all dependents of a particular store path

## See also
- nix store.3

## Categories
packages, files, security

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/nix-store.html
- tldr page: https://tldr.inbrowser.app/pages/common/nix-store.2
