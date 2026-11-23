# phpstan

Una herramienta de análisis estático de PHP para descubrir fallos en el código.

## Usage
- `phpstan analyse ruta/al/directorio1 ruta/al/directorio2 ...` : analiza uno o más directorios
- `phpstan analyse ruta/al/directorio -c ruta/a/configuración` : analiza un directorio utilizando un archivo de configuración
- `phpstan analyse ruta/al/directorio -l 1..10` : analiza usando un nivel de regla específico (0-10, más alto es más estricto)
- `phpstan analyse ruta/al/directorio -a ruta/archivo/archivo_autocarga` : especifica un archivo de carga automática para cargar antes de analizar
- `phpstan analyse ruta/al/directorio --memory-limit límite_memoria` : especifica un límite de memoria durante el análisis
- `phpstan analyse --help` : muestra las opciones disponibles para el análisis

## Links
- More information: https://phpstan.org/user-guide/command-line-usage
