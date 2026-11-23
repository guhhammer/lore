# aura

El gestor de paquetes Aura: un gestor de paquetes seguro y multilingüe para Arch Linux y el AUR.

## Usage
- `aura --aursync --both --search palabra_clave|expresión_regular` : busca paquetes en los repositorios oficiales y AUR
- `aura --aursync paquete` : instala un paquete desde el AUR
- `aura --aursync --diff --sysupgrade --delmakedeps --unsuppress` : actualiza todos los paquetes del AUR en modo detallado y elimina todas las dependencias de construcción
- `aura --sync paquete` : instala un paquete desde los repositorios oficiales
- `aura --sync --refresh --sysupgrade` : sincroniza y actualiza todos los paquetes desde los repositorios oficiales
- `aura --downgrade paquete` : reemplaza un paquete con uno más antiguo usando la caché de paquetes
- `aura --remove --recursive --unneeded paquete` : elimina un paquete y sus dependencias
- `aura --orphans --abandon` : elimina paquetes huérfanos (instalados como dependencias pero no requeridos por ningún paquete)

## Links
- More information: https://github.com/fosskers/aura
