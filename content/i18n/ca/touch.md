# touch

Canvia els temps d'accés i modificació d'un fitxer (atime, ntime).

## Usage
- `touch camí/al/fitxer` : crea un o múltiples fitxers o canvia els temps al temps actual
- `touch -t YYYYMMDDHHMM.SS camí/al/fitxer` : estableix el temps d'un fitxer a una data i hora específica
- `touch -d "-1 hour" camí/al/fitxer` : estableix el temps en un fitxer a fa una hora
- `touch -r camí/al/fitxer1 camí/al/fitxer2` : fa servir el temps d'un fitxer per establir el temps d'un segons fitxer
- `touch camí/al/fitxer{1,2,3}.txt` : crea múltiples fitxers
