# xcodebuild

Construye proyectos Xcode.

## Usage
- `xcodebuild -workspace nombre_del_espacio_de_trabajo.workspace -scheme nombre_scheme -configuration nombre_configuration clean build SYMROOT=ruta_SYMROOT` : construye espacio de trabajo
- `xcodebuild -target nombre_target -configuration nombre_configuration clean build SYMROOT=ruta_SYMROOT` : construye proyecto
- `xcodebuild -showsdks` : muestra los SDKs

## Links
- More information: https://developer.apple.com/library/archive/technotes/tn2339/_index.html
