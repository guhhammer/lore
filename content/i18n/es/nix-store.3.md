# nix-store.3

Manipula el almacén de Nix.

## Usage
- `nix store gc` : recolecta basura, es decir, elimina rutas desusadas para reducir el uso de disco
- `nix store optimise` : crea un enlace físico entre archivos idénticos para reducir el uso de disco
- `nix store delete /nix/store/...` : elimina una ruta específica en el almacén (debe ser desusada)
- `nix store --store https://cache.nixos.org ls /nix/store/...` : lista el contenido de una ruta en el almacén, en un almacén remoto
- `nix store diff-closures /nix/store/... /nix/store/...` : muestra las diferencias de versiones entre dos rutas del almacén, con sus dependencias respectivas

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/new-cli/nix3-store.html
