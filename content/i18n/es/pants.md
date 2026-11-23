# pants

Herramienta de flujo de trabajo rápida, escalable, fácil de usar y de código abierto.

## Usage
- `pants list ::` : lista todos los objetivos
- `pants test ::` : ejecuta todas las pruebas
- `pants --changed-since=HEAD fix fmt lint` : arregla, formatea y limpia sólo los archivos no comprometidos
- `pants --changed-since=HEAD --changed-dependents=transitive check` : comprueba sólo los archivos no comprometidos y sus dependientes
- `pants package ruta/al/directorio:nombre-destino` : crea un paquete distribuible para el objetivo especificado
- `pants tailor ::` : autogenera objetivos de archivo BUILD para nuevos archivos fuente
- `pants help` : muestra la ayuda

## Links
- More information: https://www.pantsbuild.org/stable/docs/using-pants/command-line-help
