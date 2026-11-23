# nix run

Run an application from a Nix flake.

## Usage
- `nix run` : run the default application in the flake in the current directory
- `nix run nixpkgs#pkg` : run a command whose name matches the package name from nixpkgs (if you want a different command from that package, see `tldr nix shell`)
- `nix run nixpkgs#vim -- path/to/file` : run a command with provided arguments
- `nix run remote_name:owner/repository` : run from a remote repository
- `nix run remote_name:owner/repository/reference` : run from a remote repository using a specific tag, revision, or branch
- `nix run "remote_name:owner/repository?dir=dir_name#app"` : run from a remote repository specifying a subdirectory and a program
- `nix run github:owner/repository/pull/number/head` : run the flake of a GitHub pull request

## See also
- nix flake

## Categories
packages, files

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/new-cli/nix3-run.html
- tldr page: https://tldr.inbrowser.app/pages/common/nix-run
