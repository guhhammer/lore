# apt-file

Cerca file nei pacchetti `apt`, inclusi quelli non ancora installati.

## Usage
- `sudo apt update` : aggiorna il database dei metadati
- `apt-file find percorso/del/file` : cerca i pacchetti che contengono il file o percorso specificato
- `apt-file list pacchetto` : elenca i contenuti di un pacchetto specifico
- `apt-file find -x regex` : cerca pacchetti che corrispondono a una `regex`

## Links
- More information: https://manned.org/apt-file
