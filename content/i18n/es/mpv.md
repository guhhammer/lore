# mpv

Reproductor de audio/vídeo basado en MPlayer.

## Usage
- `mpv url|ruta/al/archivo` : reproduce un vídeo o audio desde una URL o un archivo
- `<ArrowLeft>|<ArrowRight>` : avanza o retrocede 5 segundos
- `<ArrowLeft>|<ArrowRight>` : avanza o retrocede 1 minuto
- `<[>|<]>` : disminuye o aumenta la velocidad de reproducción en un 10 %
- `mpv --sub-file=ruta/al/archivo` : añade subtítulos desde un archivo
- `<s>` : hace una captura de pantalla del fotograma actual (guardada en `mpv-shotNNNN.jpg` por defecto)
- `mpv --speed 0.01..100 ruta/al/archivo` : reproduce un archivo a una velocidad específica (1 por defecto)
- `mpv --profile nombre_del_perfil ruta/al/archivo` : reproduce un archivo utilizando un perfil definido en el archivo `mpv.conf`

## Links
- More information: https://mpv.io/manual/stable/
