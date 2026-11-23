# dotnet build

Compila una aplicación .NET y sus dependencias.

## Usage
- `dotnet build` : compila el proyecto o solución en el directorio actual
- `dotnet build ruta/al/proyecto_o_solución` : compila un proyecto o solución .NET en el modo de depuración
- `dotnet build -c Release` : compila en modo de lanzamiento
- `dotnet build --no-restore` : compila sin restaurar las dependencias
- `dotnet build -v quiet|minimal|normal|detailed|diagnostic` : compila con un nivel específico de verbosidad
- `dotnet build -r identificador_del_tiempo_de_ejecución` : compila para un tiempo de ejecución específico
- `dotnet build -o ruta/al/directorio` : especifica el directorio de salida

## Links
- More information: https://learn.microsoft.com/dotnet/core/tools/dotnet-build
