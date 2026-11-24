# nix eval

Evaluate a Nix expression and print the result to `stdout`.

## Usage
- `nix eval .#attributes` : evaluate Nix flake attributes in the current directory
- `nix eval --expr 'your_expression'` : evaluate a given Nix expression
- `nix eval --file path/to/file` : evaluate a Nix expression from a specified file
- `nix eval nixpkgs#pkg --raw` : print the store path of the specified package from nixpkgs
- `nix eval github:owner/repo#attributes` : evaluate attributes from a flake directly from GitHub
- `nix eval nixpkgs#pkg --apply 'lambda_function'` : evaluate a given lambda function passing the specified package as argument

## Categories
files, documents

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/new-cli/nix3-eval.html
- tldr page: https://tldr.inbrowser.app/pages/common/nix-eval
- Wikipedia: https://en.wikipedia.org/wiki/Nix_(package_manager)
