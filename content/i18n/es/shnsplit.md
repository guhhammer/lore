# shnsplit

Divide archivos de audio según un archivo `.cue`. Más información: <http://shnutils.freeshell.org/shntool/>.

## Usage
- `shnsplit -f ruta/al/archivo.cue ruta/al/archivo.wav` : divide un archivo `.wav` + `.cue` en múltiples archivos
- `shnsplit -a` : muestra formatos compatibles
- `shnsplit -f ruta/al/archivo.cue -o flac ruta/al/archivo.flac` : divide un archivo `.flac` en varios archivos
- `shnsplit -f ruta/al/archivo.cue ruta/al/archivo.wav -t "%n - %a - %t"` : divide un archivo `.wav` en archivos de la forma "número-de-pista - álbum - título"
