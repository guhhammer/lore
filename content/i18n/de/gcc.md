# gcc

Präprozessiert und kompiliert C und C++ Quellcodedateien und linkt diese anschließend zusammen.

## Usage
- `gcc pfad/zu/datei1.c pfad/zu/datei2.c -o pfad/zu/binärdatei` : kompiliere mehrere Quellcodedateien zu einer ausführbaren Datei
- `gcc pfad/zu/datei.c -Wall -Og -o pfad/zu/binärdatei` : erlaube Warnungen und debug-Symbole in der Ausgabedatei
- `gcc pfad/zu/datei.c -o pfad/zu/binärdatei -Ipfad/zu/headerdatei -Lpfad/zu/bibliothek1 -lpfad/zu/bibliothek2` : inkludiere Bibliotheken aus anderen Verzeichnissen
- `gcc -S pfad/zu/datei.c` : kompiliere Quellcodedateien zu Assemblerinstruktionen
- `gcc -c pfad/zu/datei.c` : kompiliere eine oder mehrere Quellcodedateien ohne diese zu linken

## Links
- More information: https://gcc.gnu.org/onlinedocs/gcc/
