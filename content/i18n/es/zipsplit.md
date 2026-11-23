# zipsplit

Divide un archivo Zip en archivos Zip más pequeños.

## Usage
- `zipsplit ruta/al/archivo.zip` : divide un archivo Zip en partes no mayores a 36000 bytes (36 MB)
- `zipsplit -n size ruta/al/archivo.zip` : usa un número dado de bytes como límite parcial
- `zipsplit -p -n size ruta/al/archivo.zip` : pausa entre la creación de cada parte
- `zipsplit -b ruta/al/directorio_de_salida -n size ruta/al/archivo.zip` : da salida a los archivos Zip más pequeños en un directorio dado

## Links
- More information: https://manned.org/zipsplit
