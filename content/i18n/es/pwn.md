# pwn

Biblioteca de desarrollo de exploits diseñada para la creación rápida de prototipos.

## Usage
- `pwn asm "xor edi, edi"` : convierte código ensamblador a `bytes`
- `pwn cyclic número` : crea un patrón cíclico con un número específico de caracteres
- `pwn hex deafbeef` : codifica datos en el sistema hexadecimal
- `pwn unhex 6c4f7645` : decodifica datos en hexadecimal
- `pwn shellcraft amd64.linux.sh` : imprime código de intérprete de Linux x64 para ejecutar en un intérprete
- `pwn checksec ruta/al/archivo` : comprueba la configuración de seguridad binaria del archivo ELF dado
- `pwn update` : busca actualizaciones de Pwntools
- `pwn version` : muestra la versión

## Links
- More information: https://docs.pwntools.com/en/stable/commandline.html
