# gcc

Preprocesa y compila archivos de código fuente C y C++, luego los ensambla y los une. Parte de GCC Colección de Compilación GNU (GNU Compiler Collection).

## Usage
- `gcc ruta/a/la/fuente1.c ruta/a/la/fuente2.c ... -o ruta/al/ejecutable` : compila varios archivos de código fuente en un ejecutable
- `gcc ruta/a/la/fuente.c -Wall -o ejecutable` : muestra todos los errores y advertencias
- `gcc ruta/a/la/fuente.c -Wall -g -Og -o ruta/al/ejecutable` : muestra las advertencias comunes, añade símbolos de depuración en el ejecutable, y optimiza sin afectar la depuración
- `gcc ruta/a/la/fuente.c -o ruta/al/ejecutable -Iruta/a/header -Lruta/a/la/biblioteca -lnombre_de_biblioteca` : incluye las bibliotecas de una ruta diferente
- `gcc -S ruta/a/la/fuente.c` : compila el código fuente a instrucciones de Ensamblador (Assembler)
- `gcc -c ruta/a/la/fuente.c` : compila el código fuente a un archivo objeto sin vincular
- `gcc ruta/a/la/fuente.c -O1|2|3|fast -o ruta/al/ejecutable` : optimiza el programa compilado en función de velocidad de ejecución
- `gcc --version` : muestra la versión

## Links
- More information: https://gcc.gnu.org/onlinedocs/gcc/
