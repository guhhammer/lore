# g++

Preprocesa y compila archivos de código fuente de C++. Parte de GCC (GNU Compiler Collection).

## Usage
- `g++ ruta/al/código_fuente1.cpp ruta/al/código_fuente2.cpp ... -o ruta/al/ejecutable` : compila varios archivo de código fuente en un ejecutable
- `g++ ruta/al/código_fuente.cpp -Wall -o ejecutable` : activa todos los errores y advertencias
- `g++ ruta/al/código_fuente.cpp -Wall -g -Og -o ruta/al/ejecutable` : muestra advertencias comunes, añade símbolos de depuración a la salida y optimiza sin afectar la depuración
- `g++ ruta/al/código_fuente.cpp -std=c++98|c++11|c++14|c++17 -o ruta/al/ejecutable` : elija un lenguaje estándar para compilar (C++98/C++11/C++14/C++17)
- `g++ ruta/al/código_fuente.cpp -o ruta/al/ejecutable -Iruta/al/encabezado -Lruta/a/la/biblioteca -lnombre_de_la_biblioteca` : incluye bibliotecas de una ruta diferente
- `g++ -c ruta/al/código_fuente1.cpp ruta/al/código_fuente2.cpp ... && g++ -o ruta/al/ejecutable ruta/al/código_fuente1.o ruta/al/código_fuente2.o ...` : enlaza y compila múltiples archivos en un ejecutable
- `g++ ruta/al/código_fuente.cpp -O1|2|3|fast -o ruta/al/ejecutable` : optimiza el programa compilado para mejorar la velocidad de ejecución
- `g++ --version` : muestra la versión

## Links
- More information: https://gcc.gnu.org/onlinedocs/gcc/C_002b_002b-Dialect-Options.html
