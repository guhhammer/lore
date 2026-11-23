# renice

Modifica la prioridad/niceness de los procesos en ejecución. Los valores de niceness van de -20 (más favorable al proceso) a 19 (menos favorable al proceso).

## Usage
- `renice -n 3 -p pid` : aumenta/disminuye la prioridad del proceso
- `renice -n -4 -u uid|usuario` : aumenta/disminuye la prioridad de todos los procesos de un usuario
- `renice -n 5 -g grupo_de_procesos` : aumenta/disminuye la prioridad de todos los procesos que pertenecen a un grupo

## Links
- More information: https://manned.org/renice.1p
