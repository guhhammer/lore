# binwalk

Strumento per l'analisi di file binari.

## Usage
- `binwalk percorso/del/file` : scansiona un file binario
- `binwalk -e -C directory_di_output percorso/del/file` : estrae file da un binario, specificando la directory di output
- `binwalk -e -M -d 2 percorso/del/file` : estrae file in maniera ricorsiva a partire da un binario, limitando la profondità di ricorsione a 2 livelli
- `binwalk -D 'png image:png' percorso/del/file` : estrae file da un binario utilizzando una particolare firma (ad esempio il MIME Type)
- `binwalk -E -J percorso/del/file` : analizza l'entropia di un binario e salva il grafico con lo stesso filename del binario, con l'estensione `.png` in fondo
- `binwalk -E -B -A percorso/del/file` : combina analisi di entropia, firme e opcode in un unico comando

## Links
- More information: https://manned.org/binwalk
