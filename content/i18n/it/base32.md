# base32

Codifica o decodifica file o `stdin` in Base32 su `stdout`.

## Usage
- `base32 percorso/del/file` : codifica un file
- `base32 -w 0|76|... percorso/del/file` : avvolgi l'output codificato a una larghezza specifica (`0` disabilita l'avvolgimento)
- `base32 -d percorso/del/file` : decodifica un file
- `comando | base32` : codifica da `stdin`
- `comando | base32 -d` : decodifica da `stdin`

## Links
- More information: https://manned.org/base32
