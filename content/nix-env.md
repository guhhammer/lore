# nix-env

Manipulate or query Nix user environments.

## Usage
- `nix-env -q` : list all installed packages
- `nix-env -q search_pattern` : query installed packages (`regex` is supported)
- `nix-env -qa search_pattern` : query available packages from the Nixpkgs registry
- `nix-env -iA nixpkgs.pkg_name` : install a package from the Nixpkgs registry
- `nix-env -i pkg_name -f example.com` : install a package from a custom URL
- `nix-env -e pkg_name` : uninstall a package
- `nix-env -u pkg_name` : upgrade a package
- `nix-env --help --option_name` : get usage help for a specific operation (`--install`, `--upgrade`, etc.)

## Categories
packages, databases, network

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/nix-env.html
- tldr page: https://tldr.inbrowser.app/pages/common/nix-env
