# nix

Un potente gestor de paquetes que hace la gestión de paquetes fiable, reproducible y declarativa. `nix` es experimental y requiere permitir funcionalidades experimentales. Algunos subcomandos como `build`, `develop`, `flake`, `registry`, `profile`, `search`, `repl`, `store`, `edit`, `why-depends`, etc. tienen sus propias páginas.

## Usage
- `mkdir -p ~/.config/nix; echo 'experimental-features = nix-command flakes' >~/.config/nix/nix.conf` : habilita el comando `nix`
- `nix search nixpkgs término_de_búsqueda` : busca un paquete en nixpkgs usando su nombre o descripción
- `nix shell nixpkgs#pkg1 nixpkgs#pkg2 nixpkgs#pkg3 ...` : inicia un shell con unos paquetes de nixpkgs disponibles
- `nix profile install nixpkgs#pkg1 nixpkgs#pkg2 nixpkgs#pkg3 ...` : instala unos paquetes de nixpkgs de manera permanente
- `nix store gc` : quita rutas desusadas del almacén de Nix para liberar espacio
- `nix repl` : inicia un entorno interactivo para evaluar expresiones de Nix
- `nix help subcomando` : muestra ayuda para cada subcomando específico

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/new-cli/nix
