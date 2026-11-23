# semanage permissive

Gestiona dominios SELinux en modo permisivo de forma persistente. Ten en cuenta que esto, en la práctica, hace que el proceso quede sin restricciones. Para un uso a largo plazo, se recomienda configurar SELinux correctamente.

## Usage
- `sudo semanage permissive -l` : muestra todos los tipos de proceso (también conocidos como dominios) que se encuentran en modo permisivo
- `sudo semanage permissive -a httpd_t` : establece el modo permisivo para un dominio
- `sudo semanage permissive -d httpd_t` : desactiva el modo permisivo para un dominio

## Links
- More information: https://manned.org/semanage-permissive
