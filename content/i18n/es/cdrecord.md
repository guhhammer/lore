# cdrecord

Graba datos en CD o DVD. Algunas opciones de cdrecord pueden provocar acciones destructivas, como borrar todos los datos de un disco.

## Usage
- `cdrecord --devices` : muestra las unidades ópticas disponibles para `cdrecord`
- `cdrecord dev=/dev/optical_drive -audio track*.cdaudio` : graba un disco solamente de audio
- `cdrecord -eject dev=/dev/optical_drive -data archivo.iso` : graba un archivo en un disco y lo expulsa una vez finalizado (algunas grabadoras lo requieren)
- `cdrecord -tao dev=/dev/optical_drive -data archivo.iso` : graba un archivo en el disco de una unidad óptica, con la posibilidad de escribir en varios discos sucesivamente

## Links
- More information: https://manned.org/cdrecord
