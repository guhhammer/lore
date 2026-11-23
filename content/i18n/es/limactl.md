# limactl

Administrador de máquinas virtuales para huéspedes Linux, con múltiples plantillas para MV (Máquinas virtuales) disponibles. Se puede utilizar para ejecutar contenedores en macOS, pero también para casos de uso genéricos de máquinas virtuales en anfitriones macOS y Linux.

## Usage
- `limactl list` : lista MVs (Máquinas virtuales)
- `limactl create --name nombre_de_la_mv template:debian|fedora|ubuntu|...` : crea una MV usando la configuración predeterminada y opcionalmente proporciona un nombre y/o una plantilla (vea `limactl create --list-templates` para plantillas disponibles)
- `limactl start nombre_de_la_mv` : inicia una MV (esto puede instalar algunas dependencias en la misma y tomar unos minutos)
- `limactl shell nombre_de_la_mv` : abre un intérprete de comandos dentro de una MV
- `limactl shell nombre_de_la_mv comando` : ejecuta un comando dentro de una MV
- `limactl stop nombre_de_la_mv` : detiene/apaga una MV
- `limactl remove nombre_de_la_mv` : suprime una MV

## Links
- More information: https://github.com/lima-vm/lima
