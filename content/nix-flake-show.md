# nix flake show

Show outputs provided by a flake.

## Usage
- `nix flake show` : show all the outputs of the flake in the current directory
- `nix flake show github:owner/repo --json --no-pretty` : show all the outputs of a flake on GitHub and print the output as a JSON on a single line
- `nix flake show github:owner/repo --json --pretty --legacy` : show all the `legacyPackages` outputs of a flake on GitHub and print the output as a multi-line indented JSON
- `nix flake show templates` : list all available flake templates for `nix flake init`

## Categories
text

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/new-cli/nix3-flake-show.html
- tldr page: https://tldr.inbrowser.app/pages/common/nix-flake-show
- Wikipedia: https://en.wikipedia.org/wiki/Nix_(package_manager)
