# xxd

Crea una representación hexadecimal (hexdump) a partir de un archivo binario, o viceversa.

## Usage
- `xxd archivo_de_entrada` : genera un hexdump a partir de un archivo binario y muestra el resultado
- `xxd archivo_de_entrada archivo_de_salida` : genera un volcado hexadecimal a partir de un archivo binario y lo guarda como un archivo de texto
- `xxd -a archivo_de_entrada` : muestra una salida más compacta, sustituyendo los ceros consecutivos (si los hay) por un asterisco
- `xxd -c 10 archivo_de_entrada` : muestra la salida con 10 columnas de un octeto (byte) cada una
- `xxd -l 32 archivo_de_entrada` : muestra la salida solo hasta una longitud de 32 bytes
- `xxd -p archivo_de_entrada` : muestra la salida en modo plano, sin espacios entre las columnas
- `xxd -r -p archivo_de_entrada archivo_de_salida` : convierte un volcado hexadecimal de texto sin formato de nuevo a binario y lo guarda como un archivo binario

## Links
- More information: https://manned.org/xxd
