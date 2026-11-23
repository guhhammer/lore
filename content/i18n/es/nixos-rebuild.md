# nixos-rebuild

Reconfigura una máquina de NixOS.

## Usage
- `sudo nixos-rebuild switch` : construye y cambia a la configuración nueva, haciéndola la predeterminada al arrancar el sistema
- `sudo nixos-rebuild switch -p nombre` : construye y cambia a la configuración nueva, haciéndola la predeterminada al arrancar el sistema (con un nombre)
- `sudo nixos-rebuild switch --upgrade` : construye y cambia a la configuración nueva, haciéndola la predeterminada al arrancar el sistema e instalando actualizaciones
- `sudo nixos-rebuild switch --rollback` : revierte cambios a la configuración, cambiando a la generación previa
- `sudo nixos-rebuild boot` : construye la configuración nueva y la predetermina sin cambiar a ella
- `sudo nixos-rebuild test` : construye y activa la configuración nueva, pero no la haga la entrada de arranque predeterminada (solo para finalidad de pruebas)
- `sudo nixos-rebuild build-vm` : construye la configuración nueva y la abre en un hipervisor
- `nixos-rebuild list-generations` : lista generaciones disponibles similarmente al menú del cargador de arranque

## Links
- More information: https://nixos.org/nixos/manual/#sec-changing-config
