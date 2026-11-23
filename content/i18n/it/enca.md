# enca

Rileva e converti l'encoding di file di testo.

## Usage
- `enca percorso/del/file1 percorso/del/file2 ...` : rileva l'encoding di uno o più file in base alla locale di sistema
- `enca -L linguaggio percorso/del/file1 percorso/del/file2 ...` : rileva l'encoding specificando un linguaggio nel formato di locale POSIX/C (e.g. zh_CN, en_US)
- `enca -L linguaggio -x encoding percorso/del/file1 percorso/del/file2 ...` : converti file ad uno specifico encoding
- `enca <file_originale -L linguaggio -x encoding_finale >nuovo_file` : crea una copia di un file esistente utilizzando un encoding diverso

## Links
- More information: https://manned.org/enca
