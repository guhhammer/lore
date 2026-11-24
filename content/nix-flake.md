# nix flake

Manage Nix flakes. Some subcommands such as `init`, `show`, and `info` have their own usage documentation.

## Usage
- `nix flake update` : update all inputs (dependencies) of the flake in the current directory
- `nix flake update input` : update a specific input (dependency) of the flake in the current directory
- `nix flake clone github:owner/repo#attributes` : clone a GitHub repository containing a flake
- `nix flake lock path/to/flake` : create missing lock file entries for a flake in a specified directory
- `nix flake --help` : display help

## Categories
files, documents, monitoring

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/new-cli/nix3-flake.html
- tldr page: https://tldr.inbrowser.app/pages/common/nix-flake
- Wikipedia: https://en.wikipedia.org/wiki/Nix_(package_manager)
