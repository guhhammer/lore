# tasklist

Muestra una lista de los procesos en ejecución en una máquina local o remota.

## Usage
- `tasklist` : muestra los procesos en ejecución actualmente
- `tasklist /fo table|list|csv` : muestra procesos en ejecución en un formato de salida específico
- `tasklist /m patrón_módulo` : muestra procesos que utilizan un archivo `.exe` o `.dll` específico
- `tasklist /s nombre_remoto /u usuario /p contraseña` : muestra procesos que se ejecutan en una máquina remota
- `tasklist /svc` : muestra servicios usados por cada proceso
