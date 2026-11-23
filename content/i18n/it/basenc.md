# basenc

Codifica o decodifica un file o `stdin` usando una codifica specificata, inviando il risultato a `stdout`.

## Usage
- `basenc --base64 percorso/del/file` : codifica un file con codifica base64
- `basenc -d --base64 percorso/del/file` : decodifica un file con codifica base64
- `comando | basenc --base32 -w 42` : codifica da `stdin` con codifica Base32 e 42 colonne
- `comando | basenc --base32` : codifica da `stdin` con codifica base32
