# kpackagetool6

KPackage Manager: instala, lista, elimina paquetes Plasma.

## Usage
- `kpackagetool6 --list-types` : lista todos los tipos de paquetes conocidos que se pueden instalar
- `kpackagetool6 -t tipo_paquete -i ruta/al/directorio` : instala el paquete desde un directorio
- `kpackagetool6 -t tipo_paquete -u ruta/al/directorio` : actualiza el paquete instalado desde un directorio
- `kpackagetool6 -t Plasma/Applet -l -g` : lista los plasmoides instalados (`--global` para todos los usuarios)
- `kpackagetool6 -t Plasma/Applet -r "nombre"` : elimina un plasmoide por su nombre

## Links
- More information: https://manned.org/kpackagetool6
