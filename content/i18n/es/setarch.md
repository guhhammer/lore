# setarch

Cambia la arquitectura reportada para la ejecución de un programa, utilizado principalmente para modificar el comportamiento de los programas en función de la arquitectura del sistema. Útil para pruebas de compatibilidad o para ejecutar aplicaciones heredadas.

## Usage
- `setarch i686 comando` : ejecuta un comando como si la arquitectura de la máquina fuera `i686` (útil para ejecutar aplicaciones de 32 bits en un kernel de 64 bits)
- `setarch x86_64 bash` : ejecuta un intérprete de comandos con la arquitectura `x86_64`
- `setarch linux32 -R comando` : desactiva la aleatorización del espacio de direcciones virtuales
- `setarch --list` : enumera las arquitecturas compatibles
- `setarch -h` : muestra la ayuda

## Links
- More information: https://manned.org/setarch
