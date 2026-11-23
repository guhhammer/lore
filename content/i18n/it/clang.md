# clang

Compila sorgenti C, C++ e Objective-C. Può essere usato come sostituto diretto di GCC. Parte di LLVM.

## Usage
- `clang percorso/del/sorgente1.c percorso/del/sorgente2.c ... -o percorso/dell/eseguibile_output` : compila più file sorgente in un eseguibile
- `clang percorso/del/sorgente.c -Wall -o eseguibile_output` : attiva l'output di tutti gli errori e i warning
- `clang percorso/del/sorgente.c -Wall -g -Og -o percorso/dell/eseguibile_output` : mostra i warning più comuni, i simboli di debug nell'output e ottimizza senza compromettere il debug
- `clang percorso/del/sorgente.c -o percorso/dell/eseguibile_output -Ipercorso/del/header -Lpercorso/della/libreria -lnome_libreria` : includi librerie da un percorso diverso
- `clang -S -emit-llvm percorso/del/sorgente.c -o percorso/dell/output.ll` : compila il codice sorgente in LLVM Intermediate Representation (IR)
- `clang -c percorso/del/sorgente.c` : compila il codice sorgente in un file oggetto senza fare il linking
- `clang percorso/del/sorgente.c -O1|2|3|fast -o percorso/dell/eseguibile_output` : ottimizza il programma compilato per le prestazioni
- `clang --version` : mostra la versione

## Links
- More information: https://clang.llvm.org/docs/ClangCommandLineReference.html
