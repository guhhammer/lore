# g++

Kompiliere C++ Quelldateien. Teil der GCC (GNU Compiler Collection).

## Usage
- `g++ pfad/zu/quelldatei.cpp -o pfad/zu/binärdatei` : kompiliere eine Quelldatei in eine ausführbare Binärdatei
- `g++ pfad/zu/quelldatei.cpp -Wall -o pfad/zu/binärdatei` : zeige geläufige Fehler und Warnungen an
- `g++ pfad/zu/quelldatei.cpp -std=C++98|C++11|C++14|C++17 -o pfad/zu/binärdatei` : wähle einen Sprachstandard (C++98/C++11/C++14/C++17) für das Kompilieren
- `g++ pfad/zu/quelldatei.cpp -o pfad/zu/binärdatei -Ipfad/zu/headerdatei -Lpfad/zu/bibliothek -lbibliotheks_name` : binde Bibliotheken, die sich an einem anderen Pfad als die Quelldatei befinden, ein
- `g++ -c pfad/zu/quelldatei_1.cpp pfad/zu/quelldatei_2.cpp ... && g++ -o pfad/zu/binärdatei pfad/zu/quelldatei_1.o pfad/zu/quelldatei_2.o ...` : kompiliere und linke mehrere Quelldateien in eine ausführbare Binärdatei
- `g++ --version` : gib die Version von `g++` aus

## Links
- More information: https://gcc.gnu.org/onlinedocs/gcc/C_002b_002b-Dialect-Options.html
