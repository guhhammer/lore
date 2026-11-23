# where

Muestra la ubicación de archivos que coinciden con el patrón de búsqueda. Por defecto busca en el directorio de trabajo actual y en las rutas definidas en la variable de entorno `%PATH%`.

## Usage
- `where patrón_de_archivo` : muestra la ubicación del patrón de archivo
- `where /T patrón_de_archivo` : muestra la ubicación del patrón de archivo incluyendo tamaño y fecha
- `where /R ruta\al\directorio patrón_de_archivo` : busca recursivamente el patrón de archivo en la ruta especificada
- `where /Q patrón_de_archivo` : retorna silenciosamente el código de error para la ubicación del patrón de archivo
