# lli

Ejecuta directamente programas desde el código de bits LLVM (bitcode).

## Usage
- `lli ruta/al/archivo.ll` : ejecuta un código de bits o un archivo IR
- `lli ruta/al/archivo.ll primer_argumento segundo_argumento ...` : ejecuta con argumentos de línea de comandos
- `lli -O3 ruta/al/archivo.ll` : habilita todas las optimizaciones
- `lli --dlopen=ruta/a/biblioteca.dll ruta/al/archivo.ll` : carga una biblioteca dinámica antes de vincular

## Links
- More information: https://www.llvm.org/docs/CommandGuide/lli.html
