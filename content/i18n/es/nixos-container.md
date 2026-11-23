# nixos-container

Inicia contenedores de NixOS usando contenedores de Linux.

## Usage
- `sudo nixos-container list` : lista contenedores activos
- `sudo nixos-container create nombre_del_contenedor --config-file ruta_a_la_configuracion_de_nix` : crea un contenedor de NixOS con un archivo de configuración específico
- `sudo nixos-container start|stop|terminate|destroy|status nombre_del_contenedor` : inicia, detiene, termina o destruye un contenedor específico
- `sudo nixos-container run nombre_del_contenedor -- comando argumentos_del_comando` : ejecuta un comando dentro de un contenedor activo
- `sudo $EDITOR /var/lib/container/nombre_del_contenedor/etc/nixos/configuration.nix && sudo nixos-container update nombre_del_contenedor` : actualiza la configuración de un contenedor
- `sudo nixos-container root-login nombre_del_contenedor` : entra a una sesión de shell interactivo en un contenedor que ya está activo

## Links
- More information: https://nixos.org/manual/nixos/stable/#ch-containers
