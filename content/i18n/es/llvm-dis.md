# llvm-dis

Convierte archivos LLVM de bitcode en representación intermedia (IR) LLVM legible.

## Usage
- `llvm-dis ruta/a/la/entrada.bc -o -` : convierte un archivo bitcode como LLVM IR y escribe el resultado en `stdout`
- `llvm-dis ruta/al/archivo.bc` : convierte un archivo bitcode en un archivo LLVM IR con el mismo nombre de archivo
- `llvm-dis ruta/a/la/entrada.bc -o ruta/al/resultado.ll` : convierte un archivo bitcode en LLVM IR, escribe el resultado al archivo especificado

## Links
- More information: https://www.llvm.org/docs/CommandGuide/llvm-dis.html
