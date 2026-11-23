# dotnet publish

Publica una aplicación .NET y sus dependencias en una carpeta para la implementación en un sistema de hospedaje.

## Usage
- `dotnet publish -c Release ruta/al/archivo_del_proyecto` : compila un proyecto .NET en modo de lanzamiento
- `dotnet publish -sc true -r identificador_del_entorno_en_tiempo_de_ejecución ruta/al/archivo_del_proyecto` : publica el entorno de ejecución de .NET Core con la aplicación para un entorno de ejecución específico
- `dotnet publish -r identificador_del_entorno_en_tiempo_de_ejecucución -p:PublishSingleFile=true ruta/al/archivo_del_proyecto` : empaqueta la aplicación en un archivo ejecutable único de una plataforma específica
- `dotnet publish -sc true -r identificador_del_entorno_de_tiempo_de_ejecución -p:PublishTrimmed=true ruta/al/archivo_del_proyecto` : recorta las bibliotecas no usadas para reducir el tamaño de la aplicación
- `dotnet publish --no-restore ruta/al/archivo_del_proyecto` : compila un proyecto .NET sin restaurar las dependencias
- `dotnet publish -o ruta/al/directorio ruta/al/archivo_del_proyecto` : especifica el directorio de salida

## Links
- More information: https://learn.microsoft.com/dotnet/core/tools/dotnet-publish
