# nix-shell.2

Inicia un shell interactivo basado on una expresión de Nix.

## Usage
- `nix-shell` : inicia con una expresión de nix en `shell.nix` o `default.nix` del directorio actual
- `nix-shell --run "comando argumento1 argumento2 ..."` : ejecuta un comando de shell en un shell no interactivo y sale
- `nix-shell default.nix` : ejecuta con la expresión en `default.nix` en el directorio actual
- `nix-shell -p paquete1 paquete2 ...` : inicia con paquetes cargados de nixpkgs
- `nix-shell -p paquete1 paquete2 ... -I nixpkgs=https://github.com/NixOS/nixpkgs/archive/revision_de_nixpkgs.tar.gz` : inicia con paquetes cargados desde una revisión específica de nixpkgs
- `nix-shell -i interpretador -p paquete1 paquete2 ...` : evalua el resto de un archivo en un interpretador, para usarse con `#!-scripts` (vea también <https://nixos.org/manual/nix/stable/#use-as-a-interpreter>)

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/nix-shell.html
