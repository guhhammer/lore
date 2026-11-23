# expac

Una herramienta de extracción de datos para bases de datos alpm, que ofrece una flexibilidad similar a printf para utilidades basadas en pacman.

## Usage
- `expac -S '%D' paquete` : lista las dependencias de un paquete
- `expac -S "%o" paquete` : lista las dependencias opcionales de un paquete
- `expac -S -H M '%k\t%n' paquete1 paquete2 ...` : lista el tamaño de descarga de los paquetes en MiB
- `expac -S -H M '%k\t%n' $(pacman -Qqu) | sort -sh` : lista los paquetes marcados para actualización con su tamaño de descarga
- `expac -d '\n\n' -l '\n\t' -Q '%n\n\t%O' $(pacman -Qeq)` : enumera los paquetes instalados explícitamente con sus dependencias opcionales

## Links
- More information: https://github.com/falconindy/expac#name
