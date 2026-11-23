# libtool

Een generiek script voor bibliotheekondersteuning dat de complexiteit van het gebruik van gedeelde bibliotheken verbergt achter een consistente, draagbare interface.

## Usage
- `libtool c gcc -c pad/naar/bron.c -o pad/naar/bron.lo` : compileer een bronbestand naar een `libtool`-object
- `libtool l gcc -o pad/naar/bibliotheek.lo pad/naar/bron.lo` : maak een bibliotheek of een uitvoerbaar bestand
- `libtool e gdb pad/naar/programma` : stel automatisch het bibliotheekpad in zodat een ander programma niet-geïnstalleerde door `libtool` gegenereerde programma's of bibliotheken kan gebruiken
- `libtool i cp pad/naar/bibliotheek.la pad/naar/installatiemap` : installeer een gedeelde bibliotheek
- `libtool f pad/naar/installatiemap` : voltooi de installatie van `libtool`-bibliotheken op het systeem
- `libtool u pad/naar/geïnstalleerde_bibliotheek.la` : verwijder geïnstalleerde bibliotheken of uitvoerbare bestanden
- `libtool cl rm pad/naar/bron.lo pad/naar/bibliotheek.la` : verwijder niet-geïnstalleerde bibliotheken of uitvoerbare bestanden

## Links
- More information: https://www.gnu.org/software/libtool/manual/libtool.html#Invoking-libtool
