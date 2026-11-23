# set-service

Inicia, detiene y suspende un servicio, y cambia sus propiedades. Nota: Este comando solo puede usarse a través de PowerShell.

## Usage
- `Set-Service -Name hostname -DisplayName "nombre"` : cambia el nombre para mostrar
- `Set-Service -Name service_name -StartupType Automatic` : cambia el tipo de inicio de servicios
- `Set-Service -Name service_name -Description "descripción"` : cambia la descripción de un servicio
