# qmake

Genera archivos Makefile a partir de archivos de proyecto Qt.

## Usage
- `qmake` : genera un archivo `Makefile` a partir de un archivo de proyecto en el directorio actual
- `qmake -o ruta/a/Makefile ruta/para/archivo_de_proyecto.pro` : especifica las ubicaciones del archivo `Makefile` y del archivo de proyecto
- `qmake -project` : genera un archivo de proyecto predeterminado
- `qmake && make` : compila un proyecto
- `qmake -d` : habilita el modo de depuración
- `qmake -help` : muestra la ayuda

## Links
- More information: https://doc.qt.io/qt-6/qmake-running.html
