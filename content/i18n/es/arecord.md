# arecord

Grabadora de sonido para el controlador de tarjeta de sonido ALSA.

## Usage
- `arecord -vv -f cd ruta/al/archivo.wav` : graba un fragmento en calidad "CD" (finaliza con `<Ctrl c>` cuando termines)
- `arecord -vv -f cd -d 10 ruta/al/archivo.wav` : graba un fragmento en calidad "CD", con una duración fija de 10 segundos
- `arecord -vv -f cd -t raw | lame -r - ruta/al/archivo.mp3` : graba un fragmento y lo guarda como MP3 (finaliza con `<Ctrl c>` cuando termines)
- `arecord -l` : muestra todas las tarjetas de sonido y dispositivos de audio digital
- `arecord -i` : permite una interfaz interactiva (por ejemplo, usa la `<Space>` o `<Enter>` para reproducir o pausar)
- `arecord -d 5 test-mic.wav && aplay test-mic.wav && rm test-mic.wav` : prueba tu micrófono grabando una muestra de 5 segundos y reproduciéndola

## Links
- More information: https://manned.org/arecord
