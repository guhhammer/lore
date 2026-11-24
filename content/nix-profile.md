# nix profile

Install, update, and remove packages from Nix profiles.

## Usage
- `nix profile add nixpkgs#pkg1 nixpkgs#pkg2 ...` : install packages from nixpkgs into the default profile
- `nix profile add github:owner/repo/pkg --profile path/to/directory` : install a package from a flake on GitHub into a custom profile
- `nix profile list` : list packages currently installed in the default profile
- `nix profile remove legacyPackages.x86_64-linux.pkg` : remove a package installed from nixpkgs from the default profile, by name
- `nix profile upgrade --all` : upgrade packages in the default profile to the latest available versions
- `nix profile rollback` : roll back (cancel) the latest action on the default profile

## Categories
packages

## Links
- More information: https://nix.dev/manual/nix/latest/command-ref/new-cli/nix3-profile.html
- tldr page: https://tldr.inbrowser.app/pages/common/nix-profile
- Wikipedia: https://en.wikipedia.org/wiki/Nix_(package_manager)
