# nix-build.2

构建 Nix 表达式。

## Usage
- `nix-build '<nixpkgs>' -A 包名` : 构建 Nix 表达式
- `nix-build '<nixpkgs>' -A 包名 --option sandbox true` : 构建沙盒化的 Nix 表达式（在非 NixOS 系统上）

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/nix-build.html
