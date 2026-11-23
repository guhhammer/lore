# llvm-as

Ir de Representación intermedia LLVM (`.ll`) a Bitcode de Ensamblador (`.bc`).

## Usage
- `llvm-as -o ruta/a/ensamblado.bc ruta/a/fuente.ll` : ensambla un archivo IR
- `llvm-as --module-hash -o ruta/a/ensamblado.bc ruta/a/fuente.ll` : ensambla un archivo IR e incluye un hash de módulo en el archivo bitcode producido
- `cat ruta/a/fuente.ll | llvm-as -o ruta/a/ensamblado.bc` : lee un archivo IR de `stdin` y lo ensambla

## Links
- More information: https://llvm.org/docs/CommandGuide/llvm-as.html
