# pw-cat

Reproduce y graba archivos de audio a través de PipeWire.

## Usage
- `pw-cat -p ruta/al/archivo.wav` : reproduce un archivo WAV en el destino predeterminado
- `pw-cat -pm ruta/al/archivo.mid` : reproduce un archivo MIDI
- `pw-cat -pd ruta/al/archivo.dsf` : reproduce un archivo DSD
- `pw-cat -po ruta/al/archivo.ac3` : reproduce un archivo de audio comprimido mediante passthrough (requiere integración con FFmpeg)
- `pw-cat -p -q 0..15 ruta/al/archivo.wav` : reproduce un archivo WAV con una calidad de remuestreo especificada (4 por defecto)
- `pw-cat -rm ruta/al/archivo.mid` : graba un archivo MIDI
- `pw-cat -r --volume 1.25 ruta/al/archivo.wav` : graba una muestra con un volumen del 125 %
- `pw-cat -r --rate 6000 ruta/al/archivo.wav` : graba una muestra utilizando una frecuencia de muestreo diferente

## Links
- More information: https://docs.pipewire.org/page_man_pw-cat_1.html
