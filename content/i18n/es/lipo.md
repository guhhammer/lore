# lipo

Herramienta para el manejo de binarios universales Mach-O.

## Usage
- `lipo ruta/al/archivo/binario.x86_64 ruta/al/archivo_binario.arm64e -create -output ruta/al/archivo_binario` : crea un archivo universal a partir de dos archivos de una sola arquitectura
- `lipo ruta/al/archivo_binario -archs` : lista todas las arquitecturas contenidas en un archivo universal
- `lipo ruta/al/archivo_binario -detailed_info` : muestra información detallada sobre un archivo universal
- `lipo ruta/al/archivo_binario -thin arm64e -output ruta/al/archivo_binario.arm64e` : extrae un archivo de arquitectura única de un archivo universal

## Links
- More information: https://keith.github.io/xcode-man-pages/lipo.1.html
