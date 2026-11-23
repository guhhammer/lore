# clang++

Compila archivos con código fuente C++. Hace parte de LLVM.

## Usage
- `clang++ ruta/al/código1.cpp ruta/al/código2.cpp ... -o ruta/al/ejecutable` : compila un conjunto de archivos de código fuente a un binario ejecutable
- `clang++ ruta/al/código.cpp -Wall -o ruta/al/ejecutable` : activa la visualización de todos los errores y advertencias
- `clang++ ruta/al/código.cpp -Wall -g -Og -o ruta/al/ejecutable` : muestra advertencias comunes, símbolos de depuración en la salida y optimiza sin afectar la depuración
- `clang++ ruta/al/código.cpp -std=c++20 -o ruta/al/ejecutable` : elije un estándar de lenguaje para compilar
- `clang++ ruta/al/código.cpp -o ruta/al/ejecutable -Iruta/al/directorio/de/headers -Lruta/al/directorio/de/bibliotecas -lruta/a/la/biblioteca` : incluye las bibliotecas ubicadas en una ruta distinta a la del código fuente
- `clang++ -S -emit-llvm ruta/al/código.cpp -o ruta/a/la/representación.ll` : compila el código fuente hacia una representación intermedia (IR) LLVM
- `clang++ ruta/al/código.cpp -O1|2|3|fast -o ruta/al/ejecutable` : optimiza el programa compilado en función de la velocidad
- `clang++ --version` : muestra la versión

## Links
- More information: https://clang.llvm.org/docs/UsersManual.html#command-line-options
