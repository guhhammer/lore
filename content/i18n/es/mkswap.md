# mkswap

Configura un área de intercambio de Linux en un dispositivo o en un archivo.

## Usage
- `sudo mkswap ruta/al/archivo` : configura un área de intercambio determinada
- `sudo mkswap -c ruta/al/archivo` : busca bloques defectuosos en una partición antes de crear el área de intercambio
- `sudo mkswap -L etiqueta /dev/sdXY` : especifica una etiqueta para la partición (para que `swapon` pueda usar la etiqueta)
- `sudo mkswap -U clear|random|time|valor_uuid` : usa el UUID especificado
- `sudo mkswap -s tamaño_del_archivo -F ruta/al/archivo_de_intercambio` : configura un archivo de intercambio (para btrfs, vea `tldr btrfs filesystem` en su lugar)

## Links
- More information: https://manned.org/mkswap
