# systemctl disable

Desactiva servicios de systemd.

## Usage
- `systemctl disable unidad` : evita que un servicio se ejecute al arrancar
- `systemctl disable unidad --now` : evita que un servicio se ejecute al arrancar y detiene su ejecución actual
- `systemctl disable unidad --user` : evita que un servicio de usuario se ejecute al iniciar sesión
