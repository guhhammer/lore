# nix-instantiate

Create store derivations from nix expressions.

## Usage
- `nix-instantiate path/to/file.nix` : instantiate a store derivation from a Nix file
- `nix-instantiate --eval -E expression` : evaluate an expression
- `nix-instantiate --eval --xml -E expression` : evaluate with machine readable output
- `nix-instantiate --eval --raw -E expression` : evaluate an expression and print the raw output (the output of the function must be a string)
- `nix-instantiate --eval path/to/file.nix` : evaluate a Nix expression from a specified file

## See also
- nix eval

## Categories
files

## Links
- More information: https://nix.dev/manual/nix/latest/command-ref/nix-instantiate.html
- tldr page: https://tldr.inbrowser.app/pages/common/nix-instantiate
