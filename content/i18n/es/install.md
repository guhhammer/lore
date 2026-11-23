# install

Copia archivos y establecer atributos. Usualmente utilizado en archivos Makefile.

## Usage
- `install ruta/al/archivo_fuente1 ruta/al/archivo_fuente2 ... ruta/al/destino` : copia archivos al destino
- `install -o usuario ruta/al/archivo_de_origen1 ruta/al/archivo_de_origen2 ... ruta/al/destino` : copia archivos al destino, estableciendo su propiedad
- `install -g usuario ruta/al/archivo_de_origen1 ruta/al/archivo_de_origen2 ... ruta/al/destino` : copia archivos al destino, estableciendo su propiedad de grupo
- `install -m +x ruta/al/archivo_de_origen1 ruta/al/archivo_de_origen2 ... ruta/al/destino` : copia archivos al destino, estableciendo su `modo`
- `install -p ruta/al/archivo_de_origen1 ruta/al/archivo_de_origen2 ... ruta/al/destino` : copia archivos y aplica las fechas y horas de acceso y modificación de los archivos de origen a los de destino
- `install -D ruta/al/archivo_de_origen1 ruta/al/archivo_de_origen2 ... ruta/al/destino` : copia archivos y crea los directorios en el destino si no existen
