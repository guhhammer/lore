# dpkg-buildpackage

Compila paquetes binarios y/o fuente en formato Debian a partir del código fuente. Normalmente se ejecuta dentro de un árbol de código fuente que contiene un directorio `debian/`. También puede verificar dependencias de compilación, generar archivos como `.buildinfo` y `.changes`, y firmar el resultado si es aplicable.

## Usage
- `dpkg-buildpackage` : genera paquetes fuente y binario
- `dpkg-buildpackage -b` : genera solo paquetes binarios (sin el paquete fuente)
- `dpkg-buildpackage -S` : genera solo el paquete fuente (sin compilar binarios)
- `dpkg-buildpackage -us -uc` : no firma los archivos `.dsc` y `.changes`
- `dpkg-buildpackage -nc` : no ejecuta `clean` antes de compilar
- `dpkg-buildpackage -rfakeroot` : usa `fakeroot` como comando para obtener privilegios de root durante la compilación
- `dpkg-buildpackage -Tclean` : ejecuta un objetivo específico de `debian/rules`

## Links
- More information: https://manned.org/dpkg-buildpackage
