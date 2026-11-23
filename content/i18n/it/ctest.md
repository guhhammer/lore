# ctest

Programma per eseguire test in progetti CMake.

## Usage
- `ctest -j4 --output-on-failure` : esegui tutti i test definiti nel progetto CMakw, eseguendo 4 job allo stesso tempo in parallelo
- `ctest -N` : mostra una lista dei test disponibili
- `ctest --output-on-failure -R '^nome_test$'` : esegui un singolo test in base al suo nome, o filtrando con una espressione regolare

## Links
- More information: https://manned.org/ctest
