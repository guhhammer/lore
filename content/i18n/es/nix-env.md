# nix-env

Manipula o consulta entornos de usuario de Nix.

## Usage
- `nix-env -q` : lista todos los paquetes instalados
- `nix-env -q término_de_búsqueda` : consulta los paquetes instalados
- `nix-env -qa término_de_búsqueda` : consulta los paquetes disponibles
- `nix-env -iA nixpkgs.nombre_del_paquete` : instala un paquete
- `nix-env -i nombre_del_paquete -f example.com` : instala un paquete desde un enlace
- `nix-env -e nombre_del_paquete` : desinstala un paquete
- `nix-env -u nombre_del_paquete` : actualiza un paquete
- `nix-env -u` : actualiza todos los paquetes

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/nix-env.html
