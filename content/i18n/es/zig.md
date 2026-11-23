# zig

El compilador Zig y la cadena de herramientas.

## Usage
- `zig build` : compila el proyecto en el directorio actual
- `zig build run` : compila y ejecuta el proyecto en el directorio actual
- `zig init` : inicializa un proyecto `zig build` con biblioteca y ejecutable
- `zig test ruta/al/archivo.zig` : crea y ejecuta una compilación de pruebas
- `zig build run -fwine -Dtarget=x86_64-windows` : hace compilación cruzada, arma y ejecuta un proyecto para la arquitectura `x86_64` y el sistema operativo `windows`
- `zig fmt ruta/al/archivo.zig` : reformatea código fuente Zig en forma canónica
- `zig translate-c -lc ruta/al/archivo.c` : traduce un archivo C a `zig`
- `zig c++ ruta/al/archivo.cpp` : usa Zig como compilador de C++

## Links
- More information: https://ziglang.org/documentation/master/
