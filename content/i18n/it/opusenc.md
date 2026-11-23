# opusenc

Converte audio WAV o FLAC in Opus.

## Usage
- `opusenc percorso/del/file_originale.wav percorso/del/file_convertito.opus` : converte un file WAV in un file Opus usando le opzioni predefinite
- `opusenc --bitrate 512 percorso/del/file_originale.wav percorso/del/file_convertito.opus` : converte un audio stereo alla massima qualità possibile
- `opusenc --bitrate 1536 percorso/del/file_originale.flac percorso/del/file_convertito.opus` : converte un audio con canali surround 5.1 alla massima qualità possibile
- `opusenc percorso/del/file_originale.wav --downmix-mono --bitrate 6 percorso/del/file_convertito.opus` : converte l'audio di una voce alla minima qualità possibile

## Links
- More information: https://opus-codec.org/docs/opus-tools/opusenc.html
