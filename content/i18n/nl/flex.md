# flex

Lexicale analysatorgenerator. Gegeven de specificatie voor een lexicale analysator, genereert C-code die deze implementeert.

## Usage
- `flex analyzer.l` : genereer een analysator uit een Lex-bestand en sla deze op in het bestand `lex.yy.c`
- `flex -t analyzer.l` : schrijf de analysator naar `stdout`
- `flex analyzer.l -o analyzer.c` : geef het uitvoerbestand op
- `flex -B analyzer.l` : genereer een batchscanner in plaats van een interactieve scanner
- `cc pad/naar/lex.yy.c -o executable` : compileer een C-bestand gegenereerd door Lex

## Links
- More information: https://manned.org/flex
