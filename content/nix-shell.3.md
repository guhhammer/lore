# nix-shell.3

Start a shell in which the specified packages are available.

## Usage
- `nix shell nixpkgs#pkg1 nixpkgs#packageSet.pkg2 ...` : start an interactive shell with some packages from `nixpkgs`
- `nix shell nixpkgs/nixos-21.05#pkg` : start a shell providing a package from an older version of `nixpkgs` (21.05)
- `nix shell -L` : start a shell with the "default package" from a flake in the current directory, printing build logs if any builds happen
- `nix shell github:owner/repo#pkg` : start a shell with a package from a flake on GitHub
- `nix shell nixpkgs#pkg -c some-cmd --someflag 'Some other arguments'` : run a command in a shell with a package

## See also
- nix-shell
- nix flake

## Categories
shell, packages

## Links
- More information: https://manned.org/nix3-shell
- tldr page: https://tldr.inbrowser.app/pages/common/nix-shell.3
