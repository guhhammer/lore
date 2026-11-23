# aurman

Una utilidad de Arch Linux para construir e instalar paquetes desde el repositorio de usuarios de Arch (AUR).

## Usage
- `aurman -S -y -u` : sincroniza y actualiza todos los paquetes
- `aurman -S -y -u --noedit` : sincroniza y actualiza todos los paquetes sin mostrar los cambios en los archivos `PKGBUILD`
- `aurman -S paquete` : instala un nuevo paquete
- `aurman -S --noedit paquete` : instala un nuevo paquete sin mostrar los cambios en los archivos `PKGBUILD`
- `aurman -S --noedit --noconfirm paquete` : instala un nuevo paquete sin pedir confirmación
- `aurman -S -s palabra_clave` : busca en la base de datos de paquetes una palabra clave en los repositorios oficiales y AUR
- `aurman --remove --recursive --nosave paquete` : elimina un paquete y sus dependencias
- `aurman -S -c` : limpia la caché de paquetes (usa dos banderas `--clean` para limpiar todos los paquetes)

## Links
- More information: https://github.com/polygamma/aurman#syntax
