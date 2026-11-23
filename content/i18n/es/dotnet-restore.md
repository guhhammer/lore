# dotnet restore

Restaura las dependencias y herramientas de un proyecto .NET.

## Usage
- `dotnet restore` : restaura dependencias para un proyecto o solución .NET en el directorio actual
- `dotnet restore ruta/al/proyecto_o_solución` : restaura dependencias para un proyecto o solución .NET en una ubicación específica
- `dotnet restore --no-http-cache` : restaura dependencias sin almacenar las solicitudes HTTP en caché
- `dotnet restore --force` : obliga a todas las dependencias a ser resueltas incluso si la última restauración fue exitosa
- `dotnet restore --ignore-failed-sources` : restaura dependencias usando los orígenes con error como advertencias
- `dotnet restore -v quiet|minimal|normal|detailed|diagnostic` : restaura dependencias con un nivel específico de verbosidad

## Links
- More information: https://learn.microsoft.com/dotnet/core/tools/dotnet-restore
