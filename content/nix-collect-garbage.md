# nix-collect-garbage

Delete unused and unreachable nix store paths. Generations can be listed using `nix-env --list-generations`.

## Usage
- `nix-collect-garbage -d` : delete all user related store paths unused by current generations of each profile
- `nix-collect-garbage -d --dry-run` : simulate the deletion of old store paths
- `nix-collect-garbage --delete-older-than 30d` : delete all store paths older than 30 days
- `sudo nix-collect-garbage -d` : delete all store paths unused by the current generation of each profile system-wide

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/nix-collect-garbage.html
- tldr page: https://tldr.inbrowser.app/pages/common/nix-collect-garbage
