# bunx

Ejecuta el binario de un paquete (instalado localmente u obtenido remotamente).

## Usage
- `bunx nombre_paquete "argumento_del_comando"` : descarga y ejecuta un paquete desde el registro
- `bunx nombre_paquete --version` : comprueba la versión de un paquete instalado localmente (si se encuentra)
- `bunx --bun nombre_paquete` : fuerza la ejecución de un ejecutable con el runtime de Bun (en lugar de Node)
- `bunx -p nombre_paquete comando` : ejecuta un binario que tenga un nombre distinto al de su paquete
- `bunx nombre_paquete@version "argumento_del_comando"` : descarga y ejecuta una versión específica de un paquete

## Links
- More information: https://bun.com/docs/pm/bunx
