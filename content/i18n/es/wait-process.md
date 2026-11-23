# wait-process

Espera a que los procesos se detengan antes de aceptar más entradas. Nota: Este comando solo se puede usar a través de PowerShell.

## Usage
- `Stop-Process -Id id_del_proceso; Wait-Process -Id id_del_proceso` : detiene un proceso y espera
- `Wait-Process -Name nombre_del_proceso -Timeout 30` : espera a que los procesos terminen durante un tiempo especificado
