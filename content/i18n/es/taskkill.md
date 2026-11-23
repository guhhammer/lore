# taskkill

Termina un proceso por su ID o nombre.

## Usage
- `taskkill /pid id_proceso` : termina un proceso por su ID
- `taskkill /im nombre_proceso` : termina un proceso por su nombre
- `taskkill /pid id_proceso /f` : obliga a la terminación de un proceso especificado
- `taskkill /im nombre_proceso /t` : termina un proceso y sus procesos hijos
- `taskkill /pid id_proceso /s nombre_remoto` : termina un proceso en una máquina remota
- `taskkill /?` : muestra la ayuda
