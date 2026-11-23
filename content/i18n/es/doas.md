# doas

Ejecuta un comando como otro usuario.

## Usage
- `doas comando` : ejecuta un comando como root
- `doas -u usuario comando` : ejecuta un comando como otro usuario
- `doas -s` : inicia el intérprete de comandos predeterminado como root
- `doas -C ruta/al/archivo_de_configuración comando` : analiza un archivo de configuración y comprueba si se permite la ejecución de un comando como otro usuario
- `doas -L` : hace que `doas` solicite una contraseña incluso después de haberla proporcionado anteriormente

## Links
- More information: https://man.openbsd.org/doas
