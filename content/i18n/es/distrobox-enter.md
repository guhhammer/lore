# distrobox-enter

Entra a un contenedor Distrobox. El comando ejecutado por defecto es su `$SHELL`, pero puede especificar otros o comandos enteros a ejecutar. Si se utiliza dentro de un script, una aplicación o un servicio, puede utilizar el modo `--headless` para desactivar tty e interactividad.

## Usage
- `distrobox-enter nombre_del_contenedor` : entra a un contenedor Distrobox
- `distrobox-enter nombre_del_contenedor -- sh -l` : entra a un contenedor Distrobox y ejecuta un comando al iniciar sesión
- `distrobox-enter -n nombre_del_contenedor -- uptime -p` : entra a un contenedor Distrobox sin instanciar una tty

## Links
- More information: https://distrobox.it/usage/distrobox-enter/
