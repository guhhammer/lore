# nix-shell.3

Inicia un shell en lo cual los paquetes especificados están disponibles.

## Usage
- `nix shell nixpkgs#pkg1 nixpkgs#packageSet.pkg2 ...` : inicia un shell interactivo con unos paquetes de `nixpkgs`
- `nix shell nixpkgs/nixos-21.05#pkg` : inicia un shell que provee un paquete de una versión más antigua de `nixpkgs` (21.05)
- `nix shell -L` : inicia un shell con el "paquete predeterminado" de un flake en el directorio actual, mostrando un registro de construcción si es que algo se construye
- `nix shell github:dueño/repositorio#pkg` : inicia un shell con un paquete de un flake en GitHub
- `nix shell nixpkgs#pkg -c comando --una-bandera 'otros argumentos'` : ejecuta un comando en un shell con un paquete

## Links
- More information: https://manned.org/nix3-shell
