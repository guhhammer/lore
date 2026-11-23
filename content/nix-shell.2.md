# nix-shell.2

Start an interactive shell based on a Nix expression.

## Usage
- `nix-shell` : start with nix expression in `shell.nix` or `default.nix` in the current directory
- `nix-shell --run "command argument1 argument2 ..."` : run shell command in non-interactive shell and exit
- `nix-shell default.nix` : start with expression in `default.nix` in the current directory
- `nix-shell -p package1 package2 ...` : start with packages loaded from nixpkgs
- `nix-shell -p package1 package2 ... -I nixpkgs=https://github.com/NixOS/nixpkgs/archive/nixpkgs_revision.tar.gz` : start with packages loaded from specific nixpkgs revision
- `nix-shell -i interpreter -p package1 package2 ...` : evaluate rest of file in specific interpreter, for use in `#!-scripts` (see <https://nixos.org/manual/nix/stable/#use-as-a-interpreter>)

## See also
- nix shell.3

## Categories
shell, network, development

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/nix-shell.html
- tldr page: https://tldr.inbrowser.app/pages/common/nix-shell.2
