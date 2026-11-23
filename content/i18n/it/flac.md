# flac

Codifica, decodifica e controlla file flac.

## Usage
- `flac percorso/del/file.wav` : converte un file wav in un file flac (questo creerà un file flac nella medesima posizione del file wav)
- `flac -o percorso/del/file_compresso.flac percorso/del/file_originale.wav` : codifica un file wav in flac, specificando il nome del risultato
- `flac -d -o percorso/del/file_decompresso.wav percorso/del/file_originale.flac` : decodifica un file wav in flac, specificando il nome del risultato
- `flac -t percorso/del/file.flac` : controlla che un file flac sia codificato correttamente

## Links
- More information: https://xiph.org/flac/documentation_tools_flac.html
