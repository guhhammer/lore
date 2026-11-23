# lstopo

Muestra la topología de hardware del sistema.

## Usage
- `lstopo` : muestra la topología resumida del sistema en una ventana gráfica (o imprime en consola si no se dispone de una sesión gráfica)
- `lstopo --no-factorize` : muestra la topología completa del sistema sin resúmenes
- `lstopo -p` : muestra la topología resumida del sistema sólo con índices físicos (es decir, tal y como la ve el sistema operativo)
- `lstopo --no-factorize --of console|ascii|tex|fig|svg|pdf|ps|png|xml ruta/al/archivo` : escribe la topología completa del sistema en un archivo con el formato especificado
- `lstopo --palette none|grey` : muestra datos en monocromo o escala de grises

## Links
- More information: https://manned.org/lstopo
