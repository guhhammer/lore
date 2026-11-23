# forfiles

Selecciona archivos para ejecutar un comando especificado.

## Usage
- `forfiles` : busca archivos en el directorio actual
- `forfiles /p ruta\al\directorio` : busca archivos en un directorio específico
- `forfiles /c "comando"` : ejecuta el comando especificado para cada archivo
- `forfiles /m patrón_glob` : busca archivos utilizando un patrón de `glob` específico
- `forfiles /s` : busca archivos de forma recursiva
- `forfiles /d +5` : busca archivos más antiguos que 5 días
