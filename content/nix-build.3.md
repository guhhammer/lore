# nix-build.3

Build a Nix expression (downloading from the cache when possible).

## Usage
- `nix build nixpkgs#pkg` : build a package from nixpkgs, symlinking the result to `./result`
- `nix build -L .#pkg` : build a package from a flake in the current directory, showing the build logs in the process
- `nix build path/to/directory` : build the default package from a flake in some directory
- `nix build --no-link --print-out-paths` : build a package without making the `result` symlink, instead printing the store path to the `stdout`

## See also
- nix-build
- nix flake

## Categories
development, files, packages

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/new-cli/nix3-build.html
- tldr page: https://tldr.inbrowser.app/pages/common/nix-build.3
