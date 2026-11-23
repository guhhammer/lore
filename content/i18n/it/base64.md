# base64

Codifica o decodifica file o `stdin` in Base64 su `stdout`.

## Usage
- `base64 percorso/del/file` : codifica un file
- `base64 -w 0|76|... percorso/del/file` : avvolgi l'output codificato a una larghezza specifica (`0` disabilita l'avvolgimento)
- `base64 -d percorso/del/file` : decodifica un file
- `comando | base64` : codifica da `stdin`
- `comando | base64 -d` : decodifica da `stdin`

## Links
- More information: https://manned.org/base64
