# valgrind

Programa para un conjunto de herramientas expertas con programas de perfilado, optimización y depuración. Entre las herramientas de uso común cabe citar: `memcheck`, `cachegrind`, `callgrind`, `massif`, `helgrind` y `drd`.

## Usage
- `valgrind programa` : utiliza la herramienta Memcheck (por defecto) para mostrar un diagnóstico del uso de la memoria por `programa`
- `valgrind --leak-check=full --show-leak-kinds=all programa` : utiliza Memcheck para informar sobre todas las posibles fugas de memoria de `programa` en detalle
- `valgrind --tool=cachegrind programa` : utiliza la herramienta Cachegrind para perfilar y registrar operaciones de caché de CPU de `programa`
- `valgrind --tool=massif --stacks=yes programa` : utiliza la herramienta Massif para perfilar y registrar la memoria heap y el uso de la pila de `programa`

## Links
- More information: https://valgrind.org/docs/manual/manual-core.html#manual-core.options
