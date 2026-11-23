# nix classic

Una interfaz clásica y estable para un potente gestor de paquetes que hace la gestión de paquetes fiable, reproducible y declarativa. Algunos comandos Nix como `nix-build`, `nix-shell`, `nix-env`, y `nix-store` tienen sus propias páginas.

## Usage
- `nix-env -qaP termino_busqueda_regexp` : busca un paquete en nixpkgs a través de su nombre
- `nix-shell -p pkg1 pkg2 pkg3...` : inicia un shell con los paquetes especificados disponibles
- `nix-env -iA nixpkgs.pkg1 nixpkgs.pkg2...` : instala algunos paquetes de forma permanente
- `nix-store --query --tree /nix/store/...` : muestra todas las dependencias de una ruta de almacenamiento (paquete), en formato de árbol
- `nix-channel --update` : actualiza los canales (repositorios)
- `nix-collect-garbage` : elimina rutas no utilizadas del almacén Nix

## Links
- More information: https://nixos.org/
