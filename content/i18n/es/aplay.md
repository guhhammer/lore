# aplay

Reproductor de sonido para el controlador de tarjeta de sonido ALSA.

## Usage
- `aplay ruta/al/archivo` : reproduce un archivo específico (la tasa de muestreo, la profundidad de bits, etc. se determinarán automáticamente según el formato del archivo)
- `aplay --duration=10 --rate=2500 ruta/al/archivo` : reproduce los primeros 10 segundos de un archivo específico a 2500 Hz
- `aplay --channels=1 --file-type raw --rate=22050 --format=mu_law ruta/al/archivo` : reproduce el archivo en formato sin procesar como un archivo `.au` de 22050 Hz, mono, 8 bits, Mu-Law

## Links
- More information: https://manned.org/aplay
