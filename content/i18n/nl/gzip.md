# gzip

Comprimeer/decomprimeer bestanden met `gzip`-compressie (LZ77).

## Usage
- `gzip pad/naar/bestand` : comprimeer een bestand en vervang het met een `gzip`-archief
- `gzip -d pad/naar/bestand.gz` : decomprimeer een bestand en vervang het met de originele, ongecomprimeerde versie
- `gzip -v pad/naar/bestand` : toon de naam en reductiepercentage voor elk gecomprimeerd bestand
- `gzip -k pad/naar/bestand` : comprimeer een bestand en behoud het originele bestand
- `gzip -c pad/naar/bestand >pad/naar/gecomprimeerd_bestand.gz` : comprimeer een bestand en geef de uitvoerbestandsnaam op
- `gzip -cd pad/naar/bestand.gz >pad/naar/gedecomprimeerd_bestand` : decomprimeer een `gzip`-archief en geef de uitvoerbestandsnaam op
- `gzip -1..9 -c pad/naar/bestand >pad/naar/gecomprimeerd_bestand.gz` : specificeer het compressieniveau. 1 is het snelst (lage compressie), 9 is het traagst (hoge compressie) en 6 is de standaard
- `gzip -l pad/naar/bestand.txt.gz` : toon de inhoud van een gecomprimeerd bestand

## Links
- More information: https://www.gnu.org/software/gzip/manual/gzip.html
