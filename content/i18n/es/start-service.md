# start-service

Inicia servicios detenidos. Nota: Este comando solo puede usarse a través de PowerShell.

## Usage
- `Start-Service -Name nombre_del_servicio` : inicia un servicio usando su nombre
- `Start-Service -DisplayName *nombre* -WhatIf` : muestra información sin iniciar un servicio
- `Set-Service nombre_del_servicio -StartupType manual; Start-Service nombre_del_servicio` : inicia un servicio deshabilitado
