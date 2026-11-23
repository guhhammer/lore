# clang

Compiler für C, C++ und Objective-C Quelldateien. Kann als Ersatz für GCC genutzt werden.

## Usage
- `clang pfad/zu/quelldatei.c -o pfad/zu/binärdatei` : kompiliere eine Quelldatei in eine ausführbare Binärdatei
- `clang pfad/zu/quelldatei.c -Wall -o pfad/zu/binärdatei` : zeige geläufige Fehler und Warnungen an
- `clang pfad/zu/quelldatei.c -o pfad/zu/binärdatei -Ipfad/zu/headerdatei -Lpfad/zu/bibliothek -lbibliotheks_name` : binde Bibliotheken, die sich an einem anderen Pfad als die Quelldatei befinden, ein
- `clang -S -emit-llvm pfad/zu/quelldatei.c -o pfad/zu/ir_datei.ll` : kompiliere eine Quelldatei zu LLVM Intermediate Representation (IR)
- `clang -c pfad/zu/quelldatei.c` : kompiliere eine Quelldatei, ohne zu Linken

## Links
- More information: https://clang.llvm.org/docs/ClangCommandLineReference.html
