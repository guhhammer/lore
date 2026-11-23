# sed

Rediger tekst, programmatisk.

## Usage
- `sed 's/regular_expression/erstat/' filnavn` : erstat den første forekomst af et regulært udtryk (`regex`) i hver linje af en fil, og print resultatet
- `sed -r 's/regular_expression/erstat/g' filnavn` : erstat alle forekomster af et regulært udtryk in en fil, og print resultatet
- `sed -i 's/find/erstat/g' filnavn` : erstat alle forekomster af en streng i en fil, og overskriv filen
- `sed '/linje_mønster/s/find/erstat/' filnavn` : erstat forekomster på linjer der matcher mønstret
- `sed '/linje_mønster/d' filnavn` : fjern linjer der matcher mønstret
- `sed 11q filnavn` : print de første 11 linjer af en fil
- `sed -e 's/find/erstat/' -e 's/find/erstat/' filnavn` : udfør flere find-og-erstat udtryk i en fil
- `sed 's#find#erstat#' filnavn` : erstat separator `/` med en hvilken som helst anden karakter ikke brugt i find- eller erstat-mønstrene, f.eks. `#`

## Links
- More information: https://manned.org/sed.1posix
