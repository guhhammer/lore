# llc

Compila Representación intermedia LLVM o código bit (bitcode) para el lenguaje ensamblador objetivo específico.

## Usage
- `llc ruta/al/archivo.ll` : compila un bitcode o archivo IR a un archivo ensamblador con el mismo nombre base
- `llc -O3 ruta/al/archivo.ll` : habilita todas las optimizaciones
- `llc --output ruta/al/resultado.s` : dirige la salida a un archivo específico
- `llc -relocation-model=pic ruta/a/la/entrada.ll` : emite código, independiente de la posición que pueda reubicarse completamente

## Links
- More information: https://www.llvm.org/docs/CommandGuide/llc.html
