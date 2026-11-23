# wmic

Consola interactiva para obtener información detallada sobre los procesos en ejecución.

## Usage
- `wmic alias cláusula_where cláusula_verb` : gramática fundamental
- `wmic process list brief` : muestra detalles breves sobre los procesos en ejecución actualmente
- `wmic process list full` : muestra detalles completos sobre los procesos en ejecución actualmente
- `wmic process get name,processid,parentprocessid` : accede a campos específicos como nombre del proceso, ID del proceso e ID del proceso padre
- `wmic process where name="ejemplo.exe" list full` : muestra información sobre un proceso específico
- `wmic process where processid=pid get name,commandline` : muestra campos específicos para un proceso específico
- `wmic process pid delete` : termina un proceso
