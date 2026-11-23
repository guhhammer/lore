# musescore

MuseScore editor de música de 3 hojas.

## Usage
- `musescore --audio-driver jack|alsa|portaudio|pulse` : utiliza un controlador de audio específico
- `musescore --bitrate bitrate` : establece el bitrate de salida MP3 en kbit/s
- `musescore --debug` : inicia MuseScore en modo depuración
- `musescore --experimental` : permite características experimentales, como capas
- `musescore --export-to archivo_de_salida archivo_de_entrada` : exporta el archivo dado al archivo de salida especificado. El tipo de archivo depende de la extensión dada
- `musescore --diff ruta/al/archivo1 ruta/al/archivo2` : imprime las diferencias entre las puntuaciones (scores) dadas
- `musescore --midi-operations ruta/al/archivo` : especifica un archivo de operaciones de importación MIDI

## Links
- More information: https://handbook.musescore.org/appendix/command-line-usage
