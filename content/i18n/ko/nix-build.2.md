# nix-build.2

Nix 표현식을 빌드.

## Usage
- `nix-build '<nixpkgs>' -A firefox` : nix 표현식 빌드
- `nix-build '<nixpkgs>' -A firefox --option sandbox true` : 샌드박스된 Nix 표현식 빌드 (NixOS가 아닌 경우)

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/nix-build.html
