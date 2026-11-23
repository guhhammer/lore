# radare2

Un conjunto de herramientas de ingeniería inversa.

## Usage
- `radare2 -nw ruta/al/binario` : abre un archivo en modo escritura sin analizar los encabezados del formato de archivo
- `radare2 -d ruta/al/binario` : depura un programa
- `radare2 -i ruta/a/script.r2 ruta/al/binario` : ejecuta un script antes de entrar en la CLI interactiva
- `radare2_comando?` : [Interactivo] Muestra texto de ayuda para cualquier comando en la CLI interactiva
- `!shell_comando` : [Interactivo] Ejecuta un comando desde la CLI interactiva
- `pr >ruta/al/archivo.bin` : [Interactivo] Vierte los bytes crudos del bloque actual a un archivo

## Links
- More information: https://book.rada.re/first_steps/commandline_flags.html
