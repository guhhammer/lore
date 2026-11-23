# open

Abre archivos, directorios y aplicaciones.

## Usage
- `open archivo.ext` : abre un archivo con la aplicación asociada
- `open -a "aplicacion"` : ejecuta una aplicación gráfica de macOS
- `open -b com.domain.application` : ejecuta una aplicación gráfica de macOS basada en el identificador bundle (consulta `osascript` para obtenerlo fácilmente)
- `open .` : abre el directorio actual en Finder
- `open -R ruta/al/archivo` : abre un archivo en Finder
- `open *.ext` : abre todos los archivos de una extensión determinada en el directorio actual con la aplicación asociada
- `open -n -b com.domain.application` : abre una nueva instancia de una aplicación especificada mediante un identificador bundle

## Links
- More information: https://keith.github.io/xcode-man-pages/open.1.html
