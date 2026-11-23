# cmake

Generatore di ambienti di compilazione multipiattaforma. Genera Makefile, progetti Visual Studio o altro, in base al sistema operativo.

## Usage
- `cmake && make` : genera un Makefile ed usalo per compilare un progetto nella stessa directory dei sorgenti
- `cmake -H. -Bbuild && make -C build` : genera un makefile ed usalo per compilare un progetto in una directory "build" separata (out-of-source build)
- `cmake -i` : esegui cmake in modalità interattiva (chiederà i valori di ogni variabile invece di usare i predefiniti)

## Links
- More information: https://cmake.org/cmake/help/latest/manual/cmake.1.html
