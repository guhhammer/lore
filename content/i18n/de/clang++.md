# clang++

Kompiliert C++ Quelldateien. Teil von LLVM.

## Usage
- `clang++ pfad/zu/quelldatei.cpp -o pfad/zu/binärdatei` : kompiliere eine Quelldatei in eine ausführbare Binärdatei
- `clang++ pfad/zu/quelldatei.cpp -Wall -o pfad/zu/binärdatei` : zeige geläufige Fehler und Warnungen an
- `clang++ pfad/zu/quelldatei.cpp -std=c++20 -o pfad/zu/binärdatei` : wähle einen Sprachstandard für die Kompilation aus
- `clang++ pfad/zu/qelldatei.cpp -o pfad/zu/binärdatei -Ipfad/zu/headerdatei -Lpfad/zu/bibliothek -lbibliotheksname` : binde Bibliotheken, die sich an einem anderen Pfad als die Quelldatei befinden, ein
- `clang++ -S -emit-llvm pfad/zu/quelldatei.cpp -o pfad/zu/ir_datei.ll` : kompiliere eine Quelldatei zu LLVM Intermediate Representation (IR)

## Links
- More information: https://clang.llvm.org/docs/UsersManual.html#command-line-options
