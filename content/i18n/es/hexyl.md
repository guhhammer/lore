# hexyl

Un simple visor hexadecimal para la terminal. Utiliza salida coloreada para distinguir diferentes categorías de bytes.

## Usage
- `hexyl ruta/al/archivo` : imprime la representación hexadecimal de un archivo
- `hexyl -n n ruta/al/archivo` : imprime la representación hexadecimal de los primeros `n` bytes de un archivo
- `hexyl -r 512:1024 ruta/al/archivo` : imprime los bytes 512 a 1024 de un archivo
- `hexyl -r 1024:+512 ruta/al/archivo` : imprime 512 bytes empezando por el byte 1024

## Links
- More information: https://github.com/sharkdp/hexyl/blob/master/doc/hexyl.1.md
