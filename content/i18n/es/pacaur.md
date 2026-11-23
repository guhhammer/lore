# pacaur

Utilidad de Arch Linux para construir e instalar paquetes del repositorio de usuarios de Arch.

## Usage
- `pacaur -Syu` : sincroniza y actualiza todos los paquetes (incluye AUR)
- `pacaur -Syua` : sincroniza y actualiza solo los paquetes AUR
- `pacaur -S paquete` : instala un nuevo paquete (incluye AUR)
- `pacaur -Rs paquete` : elimina un paquete y sus dependencias (incluye paquetes AUR)
- `pacaur -Ss palabra_clave` : busca una palabra clave en la base de datos de paquetes (incluye AUR)
- `pacaur -Qs` : lista todos los paquetes instalados en este momento (incluye paquetes AUR)

## Links
- More information: https://github.com/rmarquis/pacaur#name
