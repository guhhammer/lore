# nixpkgs-review

Review pull requests in the NixOS packages repository (nixpkgs). After a successful build, a `nix-shell` with all built packages is started.

## Usage
- `nixpkgs-review pr pr_number|pr_url` : build changed packages in the specified pull request
- `nixpkgs-review pr --post-result pr_number|pr_url` : build changed packages and post a comment with a report (requires setting up a token in `hub`, `gh`, or the `$GITHUB_TOKEN` environment variable)
- `nixpkgs-review pr --print-result pr_number|pr_url` : build changed packages and print a report
- `nixpkgs-review rev HEAD` : build changed packages in a local commit
- `nixpkgs-review wip` : build changed packages that haven't been committed yet
- `nixpkgs-review wip --staged` : build changed packages that have been staged

## Categories
packages, development, shell

## Links
- More information: https://github.com/Mic92/nixpkgs-review#usage
- tldr page: https://tldr.inbrowser.app/pages/common/nixpkgs-review
