# systemctl edit

Edita archivos de unidad de systemd.

## Usage
- `sudo systemctl edit archivo_unidad` : superpone un archivo de unidad de forma no destructiva
- `sudo systemctl edit archivo_unidad -l` : edita un archivo de unidad
- `sudo systemctl edit archivo_unidad -lf` : crea un nuevo archivo de unidad
- `systemctl edit archivo_unidad --user` : superpone un archivo de unidad de usuario
