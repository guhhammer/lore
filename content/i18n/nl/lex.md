# lex

Generator voor lexicale analyzers. Gegeven de specificatie voor een lexicale analyzer, genereert C-code die deze implementeert. Opmerking: op de meeste grote besturingssystemen is dit commando een alias voor `flex`.

## Usage
- `lex analyzer.l` : genereer een analyzer van een Lex-bestand en sla deze op in het bestand `lex.yy.c`
- `lex -t analyzer.l >analyzer.c` : specificeer het uitvoerbestand
- `c99 pad/naar/lex.yy.c -o uitvoerbaar_bestand` : compileer een C-bestand dat door Lex is gegenereerd

## Links
- More information: https://manned.org/lex
