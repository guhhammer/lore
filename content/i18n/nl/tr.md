# tr

Vertaal tekens: voer vervangingen uit op basis van enkele tekens en tekensets.

## Usage
- `tr <pad/naar/bestand vind_karakter vervang_karakter` : vervang alle voorkomens van een teken in een bestand en toon het resultaat
- `echo tekst | tr vind_karakter vervang_karakter` : vervang alle voorkomens van een teken uit de uitvoer van een ander commando
- `tr <pad/naar/bestand 'abcd' 'jkmn'` : map elk teken van de eerste set naar het overeenkomstige teken van de tweede set
- `tr <pad/naar/bestand -d 'invoer_karakters'` : verwijder alle voorkomens van de opgegeven set tekens uit de invoer
- `tr <pad/naar/bestand -s 'invoer_karakters'` : comprimeer een reeks identieke tekens tot een enkel teken
- `tr <pad/naar/bestand "[:lower:]" "[:upper:]"` : vertaal de inhoud van een bestand naar hoofdletters
- `tr <pad/naar/bestand -cd "[:print:]"` : verwijder niet-afdrukbare tekens uit een bestand

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/tr-invocation.html
