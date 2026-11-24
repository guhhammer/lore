# nix repl

Start an interactive environment for evaluating Nix expressions. See <https://nixos.org/manual/nix/stable/language/index.html> for a description of the Nix expression language.

## Usage
- `nix repl` : start an interactive environment for evaluating Nix expressions
- `:lf nixpkgs` : [Interactive] Load all packages from a flake (e.g. `nixpkgs`) into scope
- `:b expression` : [Interactive] Build a package from an expression
- `:u expression` : [Interactive] Start a shell with package from the expression available
- `:s expression` : [Interactive] Start a shell with dependencies of the package from the expression available

## Categories
network, packages

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/new-cli/nix3-repl.html
- tldr page: https://tldr.inbrowser.app/pages/common/nix-repl
- Wikipedia: https://en.wikipedia.org/wiki/Nix_(package_manager)
