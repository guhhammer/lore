# xcode-select

Cambia entre diferentes versiones de Xcode y las herramientas incluidas para desarrolladores. También se utiliza para actualizar la ruta a Xcode si se mueve después de la instalación.

## Usage
- `xcode-select --install` : instala las herramientas de línea de comandos de Xcode
- `xcode-select --switch ruta/al/Xcode.app/Contents/Developer` : selecciona una ruta determinada como directorio de desarrollador activo
- `xcode-select --switch ruta/al/archivo/Xcode_file.app` : selecciona una instancia de Xcode determinada y utiliza su directorio de desarrollador como directorio activo
- `xcode-select --print-path` : muestra el directorio de desarrollador seleccionado
- `sudo xcode-select --reset` : descarta cualquier directorio de desarrolladores especificado por el usuario para que se encuentre mediante el mecanismo de búsqueda predeterminado

## Links
- More information: https://developer.apple.com/library/archive/technotes/tn2339/_index.html
