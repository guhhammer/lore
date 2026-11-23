# nix edit

Abre la expresión de Nix de un paquete Nix dentro de `$EDITOR`.

## Usage
- `nix edit nixpkgs#pkg` : abre el código fuente de la expresión Nix de un paquete de nixpkgs dentro de tu `$EDITOR`
- `EDITOR=cat nix edit nixpkgs#pkg` : vuelca el código fuente de un paquete en `stdout`

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/new-cli/nix3-edit.html
