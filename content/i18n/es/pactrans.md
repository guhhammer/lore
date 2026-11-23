# pactrans

Instala, elimina y actualiza paquetes ALPM.

## Usage
- `sudo pactrans --install nombre_del_paquete` : instala un paquete desde un repositorio
- `sudo pactrans --remove nombre_del_paquete` : elimina un paquete
- `sudo pactrans --sysupgrade` : actualiza todos los paquetes instalados
- `sudo pactrans --file ruta/al/paquete.pkg.tar.zst` : instala un archivo de paquete
- `sudo pactrans local/nombre_del_paquete_a_eliminar nombre_del_repositorio/nombre_del_paquete_a_instalar` : reemplaza un paquete instalado localmente por uno de un repositorio
- `pactrans --print-only --install nombre_del_paquete` : muestra lo que haría la transacción sin ejecutarla

## Links
- More information: https://github.com/andrewgregory/pacutils/blob/master/doc/pactrans.pod
