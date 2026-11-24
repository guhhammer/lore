# nix edit

Open the Nix expression of a Nix package in `$EDITOR`.

## Usage
- `nix edit nixpkgs#pkg` : open the source of the Nix expression of a package from nixpkgs in your `$EDITOR`
- `EDITOR=cat nix edit nixpkgs#pkg` : dump the source of a package to `stdout`

## Categories
editors, packages

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/new-cli/nix3-edit.html
- tldr page: https://tldr.inbrowser.app/pages/common/nix-edit
- Wikipedia: https://en.wikipedia.org/wiki/Nix_(package_manager)
