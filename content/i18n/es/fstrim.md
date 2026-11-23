# fstrim

Descarta los bloques no usados de un sistema de archivos montado. Solo es compatible con dispositivos de memoria flash como SSD y tarjetas microSD.

## Usage
- `sudo fstrim -a` : recorta los bloques no usados en todas las particiones montadas que lo admiten
- `sudo fstrim /` : recorta los bloques no usados en una partición especificada
- `sudo fstrim -v /` : muestra estadísticas después del recorte

## Links
- More information: https://manned.org/fstrim
