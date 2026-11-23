# basenc

Codifica o decodifica un archivo o `stdin` usando una codificación especificada, a `stdout`.

## Usage
- `basenc --base64 ruta/al/archivo` : codifica un archivo con codificación base64
- `basenc -d --base64 ruta/al/archivo` : descifra un archivo con codificación base64
- `comando | basenc --base32 -w 42` : codifica desde `stdin` con codificación Base32 con 42 columnas
- `comando | basenc --base32` : codifica desde `stdin` con codificación base32
