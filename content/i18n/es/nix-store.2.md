# nix-store.2

Manipula o busca en el almacén de Nix.

## Usage
- `nix-store --gc` : recolecta basura, por ejemplo quitar las rutas desusadas
- `nix-store --optimise` : crea un enlace físico entre archivos idénticos para reducir el uso de disco
- `nix-store --delete /nix/store/...` : elimina una ruta específica en el almacén (debe ser desusada)
- `nix-store -q --tree /nix/store/...` : muestra todas las dependencias de una ruta del almacén (paquete), en un formato de árbol
- `du -cLsh $(nix-store -q --references /nix/store/...)` : calcula el tamaño total de una ruta específica del almacén con todas las dependencias
- `nix-store -q --referrers /nix/store/...` : muestra todos los dependientes de una ruta particular del almacén

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/nix-store.html
