# llvm-bcanalyzer

Analizador de bitcode LLVM (`.bc`).

## Usage
- `llvm-bcanalyzer ruta/al/archivo.bc` : imprime estadísticas sobre un archivo bitcode
- `llvm-bcanalyzer -dump ruta/al/archivo.bc` : imprime una representación SGML y estadísticas sobre un archivo bitcode
- `cat ruta/al/archivo.bc | llvm-bcanalyzer` : lee un archivo bitcode de `stdin` y lo analiza

## Links
- More information: https://llvm.org/docs/CommandGuide/llvm-bcanalyzer.html
