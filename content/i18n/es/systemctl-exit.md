# systemctl exit

Solicita al administrador de servicios que salga.

## Usage
- `systemctl exit --user` : sale del administrador de servicios de usuario
- `systemctl exit código --user` : sale del administrador de servicios de usuario con un código de salida específico
- `systemctl exit` : solicita al administrador de servicios del contenedor que salga (equivalente a `systemctl poweroff` si no está en un contenedor)
