# nixos-option

Inspecciona una configuración de NixOS.

## Usage
- `nixos-option clave` : lista todas las subclaves de una clave de opciones proporcionadas
- `nixos-option boot.kernelModules` : lista los módulos de arranque del kernel actuales
- `nixos-option users.users.usuario.openssh.authorizedKeys.archivoDeClaves|clave` : lista claves autorizadas para un usuario específico
- `nixos-option nix.buildMachines` : lista todos los constructores remotos
- `NIXOS_CONFIG=ruta_a_configuracion.nix nixos-option clave` : lista todas las subclaves de una clave proporcionada en otra configuración de NixOS
- `nixos-option -r users.users.usuario` : muestra recursivamente todos los valores de un usuario

## Links
- More information: https://nixos.org/manual/nixos/stable/index.html#sec-modularity
