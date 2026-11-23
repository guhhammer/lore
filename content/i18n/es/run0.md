# run0

Eleva privilegios interactivamente. Similar a `sudo`, pero no es un binario SUID, la autenticación tiene lugar a través de polkit, y los comandos se invocan desde un servicio `systemd`.

## Usage
- `run0 comando` : ejecuta un comando como root
- `run0 -u usuario|uid -g nombre_de_grupo|gid comando` : ejecuta un comando como otro usuario y/o grupo

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/run0.html
