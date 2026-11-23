# cppcheck

Strumento di analisi statica per codice C/C++. Piuttosto che sugli errori di sintassi, si concentra su tipi di bug che normalmente non vengono rilevati dai compilatori.

## Usage
- `cppcheck . 2>cppcheck.log` : controlla la directory corrente ricorsivamente, mostrando il progresso a schermo e loggando i messaggi di errore in un file
- `cppcheck -q percorso/della/directory` : controlla una determinata directory ricorsivamente, senza stampare informazioni sul progresso
- `cppcheck --enable error|warning|style|performance|portability|information|all percorso/del/file.cpp` : controlla un determinato file, specificando quali test eseguire (di default, solo gli errori sono mostrati)
- `cppcheck --errorlist` : elenca i test disponibili
- `cppcheck --suppress id_test1 --suppress it_test2 percorso/del/file.cpp` : controlla un determinato file, ignorando specifici test
- `cppcheck -I include/directory_1 -I include/directory_2 .` : controlla la directory corrente, fornendo percorsi da includere per file esterni (e.g. librerie esterne)
- `cppcheck --project percorso/del/progetto.sln` : controlla un progetto Microsoft Visual Studio (`*.vcxproj`) o file solution (`*.sln`)

## Links
- More information: https://manned.org/cppcheck
