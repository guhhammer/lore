# deb-get

Funcionalidad `apt-get` para paquetes `.deb` publicados en repositorios de terceros o a través de descarga directa. Funciona con distribuciones Linux que usan `apt-get`.

## Usage
- `deb-get update` : actualiza la lista de paquetes y versiones disponibles
- `deb-get search paquete` : busca un paquete dado
- `deb-get show paquete` : muestra información sobre un paquete
- `deb-get install paquete` : instala un paquete o lo actualiza a la última versión disponible
- `deb-get remove paquete` : elimina un paquete (utilizando `purge` en su lugar, también elimina sus archivos de configuración)
- `deb-get upgrade` : actualiza todos los paquetes instalados a sus versiones más recientes disponibles
- `deb-get list` : lista todos los paquetes disponibles

## Links
- More information: https://github.com/wimpysworld/deb-get
