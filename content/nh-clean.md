# nh clean

Clean Nix profiles and delete unused and unreachable Nix store paths. Generations can be listed using `nix-env --list-generations` or `nh os info`.

## Usage
- `nh clean all -a` : ask for clean up plan confirmation, clean all profiles, and collect garbage
- `nh clean user -k number` : keep a specified number of the most recent profiles for the current user and clean the remaining profiles
- `nh clean profile path/to/profile` : clean a specific profile and collect garbage

## Links
- More information: https://github.com/nix-community/nh#usage
- tldr page: https://tldr.inbrowser.app/pages/common/nh-clean
