# dc

Een rekenmachine met willekeurige precisie. Gebruikt reverse polish notation (RPN).

## Usage
- `dc` : start een interactieve sessie
- `dc pad/naar/script.dc` : voer een script uit
- `dc -e '10 k 5 3 / p'` : bereken een uitdrukking met de gespecificeerde schaal
- `dc -e '4 5 * 17 - p'` : bereken 4 keer 5 (4 5 *), trek er 17 vanaf (17 -), en print de uitvoer
- `dc -e '7 k 5 _3 / p'` : specificeer het aantal decimalen op 7 (7 k), bereken 5 gedeeld door -3 (5 _3 /) en print
- `dc -e '100 k 5 v 1 + 2 / p'` : bereken de gulden snede, phi: stel het aantal decimalen in op 100 (100 k), wortel van 5 (5 v) plus 1 (1 +), gedeeld door 2 (2 /), en print het resultaat

## Links
- More information: https://www.gnu.org/software/bc/manual/dc-1.05/html_mono/dc.html
