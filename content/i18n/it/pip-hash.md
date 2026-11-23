# pip hash

Calcola gli hash degli archivi dei pacchetti per la verifica.

## Usage
- `pip hash percorso/del/pacchetto.whl` : genera l'hash per un file di pacchetto
- `pip hash -a sha256|sha384|sha512|... percorso/del/pacchetto.whl` : genera l'hash utilizzando uno specifico algoritmo
- `pip hash percorso/del/pacchetto1.whl percorso/del/pacchetto2.whl ...` : genera gli hash per più file
- `pip hash percorso/del/pacchetto.tar.gz` : genera l'hash per un archivio scaricato

## Links
- More information: https://pip.pypa.io/en/stable/cli/pip_hash/
