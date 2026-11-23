# nix why-depends

Muesta por qué un paquete depende de otro paquete.

## Usage
- `nix why-depends /run/current-system /nix/store/...` : muestra por qué el sistema actual de NixOS requiere una ruta del almacén específica
- `nix why-depends --derivation nixpkgs#dependiente nixpkgs#dependencia` : muestra por qué un paquete de nixpkgs requiere otro paquete como una dependencia de tiempo de construcción

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/new-cli/nix3-why-depends.html
