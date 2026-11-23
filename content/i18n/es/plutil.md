# plutil

Muestra, convierte, valida o edita archivos de listas de propiedades ("plist").

## Usage
- `plutil -p archivo1.plist archivo2.plist ...` : muestra el contenido de uno o más archivos plist en un formato legible por humanos
- `plutil -convert xml1 archivo1.plist archivo2.plist ...` : convierte uno o varios archivos plist a formato XML, sobrescribiendo los archivos originales
- `plutil -convert binary1 archivo1.plist archivo2.plist ...` : convierte uno o más archivos plist a formato binario, sobrescribiendo los archivos originales en su lugar
- `plutil -convert xml1|binary1|json|swift|objc ruta/al/archivo.plist -o ruta/al/nuevo_archivo.plist` : convierte un archivo plist a un formato diferente, escribiendo en un nuevo archivo
- `plutil -convert xml1|binary1|json|swift|objc ruta/al/archivo.plist -o -` : convierte un archivo plist a un formato diferente, escribiendo en `stdout`

## Links
- More information: https://keith.github.io/xcode-man-pages/plutil.1.html
