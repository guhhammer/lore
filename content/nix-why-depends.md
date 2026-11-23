# nix why-depends

Show why a package depends on another package.

## Usage
- `nix why-depends /run/current-system /nix/store/checksum-package-version.ext` : show why the currently running NixOS system requires a certain store path
- `nix why-depends --derivation nixpkgs#dependent nixpkgs#dependency` : show why a package from nixpkgs requires another package as a build-time dependency

## Categories
packages

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/new-cli/nix3-why-depends.html
- tldr page: https://tldr.inbrowser.app/pages/common/nix-why-depends
