# stop-service

Detiene servicios en ejecución. Nota: Este comando solo puede usarse a través de PowerShell.

## Usage
- `Stop-Service -Name nombre_del_servicio` : detiene un servicio en el equipo local
- `Stop-Service -DisplayName "nombre"` : detiene un servicio usando el nombre para mostrar
- `Stop-Service -Name nombre_del_servicio -Force -Confirm` : detiene un servicio que tiene servicios dependientes
