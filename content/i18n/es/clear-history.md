# clear-history

Elimina las entradas del historial de comandos de la sesión de PowerShell.

## Usage
- `Clear-History` : borra todo el historial de comandos de la sesión actual
- `Clear-History -CommandLine "comando"` : borra un comando por su nombre específico
- `Clear-History -CommandLine "comando1", "comando2", ...` : borra varios comandos por su nombre
- `Clear-History -Id número_id` : borra una entrada específica del historial por ID
- `Clear-History -Id id1, id2, ...` : borra varios ID
- `Clear-History -Id (id_inicial..id_final)` : borra comandos dentro de un rango de IDs
- `Clear-History -WhatIf` : muestra lo que se eliminaría
- `Clear-History -Confirm` : pide confirmación antes de borrar
