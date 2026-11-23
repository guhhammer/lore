# clang

Compila archivos fuente C, C+ y Objective-C. Se puede utilizar como un reemplazo para GCC. Parte de LLVM.

## Usage
- `clang ruta/a/fuente1.c ruta/a/fuente2.c ... -o ruta/al/ejecutable_resultante` : compila archivos de múltiples fuentes en un ejecutable
- `clang ruta/a/fuente.c -Wall -o ejecutable_resultante` : activa la salida de todos los errores y advertencias
- `clang ruta/a/fuente.c -Wall -g -Og -o ruta/al/ejecutable_resultante` : muestra advertencias comunes, depura símbolos en la salida y optimiza sin afectar la depuración
- `clang ruta/a/fuente.c -o ruta/al/ejecutable_resultante -Iruta/al/encabezado -Lruta/a/la/biblioteca -lnombre_biblioteca` : incluye bibliotecas de una ruta diferente
- `clang -S -emit-llvm ruta/a/fuente.c -o ruta/a/la/salida.ll` : compila código fuente hacia representación intermedia (IR) LLVM
- `clang -c ruta/a/fuente.c` : compila código fuente en un archivo objeto sin vincular (linking)
- `clang ruta/a/fuente.c -O1|2|3|fast -o ruta/al/ejecutable_resultante` : optimiza el programa compilado para velocidad de ejecución
- `clang --version` : muestra la versión

## Links
- More information: https://clang.llvm.org/docs/ClangCommandLineReference.html
