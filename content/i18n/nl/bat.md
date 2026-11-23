# bat

Toon en voeg bestanden samen. Een `cat` kopie met syntax highlighting en Git integratie.

## Usage
- `bat pad/naar/bestand1 pad/naar/bestand2 ...` : print de inhoud van een of meerdere bestanden met opmaak naar `stdout`
- `bat pad/naar/bestand1 pad/naar/bestand2 ... >pad/naar/doelbestand` : voeg verschillende bestanden samen in het doelbestand
- `bat -pp pad/naar/bestand` : verwijder decoraties en schakel paging uit
- `bat -H 10|5:10|:10|10:|10:+5 pad/naar/bestand` : highlight een specifieke regel of een reeks van regels met een andere achtergrondkleur
- `bat -A pad/naar/bestand` : toon niet-printbare karakters zoals spatie, tab of regeleinde
- `bat -n pad/naar/bestand` : verwijder alle decoraties behalve regelnummers in de uitvoer
- `bat -l json pad/naar/bestand.json` : highlight de syntax van een JSON-bestand door expliciet de taal in te stellen
- `bat -L` : toon alle ondersteunde talen

## Links
- More information: https://manned.org/bat
