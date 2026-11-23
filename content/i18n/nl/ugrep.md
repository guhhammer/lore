# ugrep

Ultrasnelle bestandszoeker met interactieve TUI.

## Usage
- `ugrep -Q` : open een interactieve TUI om recursief bestanden in de huidige map te zoeken (`<Ctrl z>` voor hulp)
- `ugrep "zoekpatroon"` : zoek recursief met een `regex` zoekpatroon in de huidige map naar passende bestanden
- `ugrep -n "zoekpatroon" pad/naar/bestand_of_map` : zoek een gegeven bestand of bestanden in een gegeven map en laat de passende regelnummers zien
- `ugrep -l "zoekpatroon"` : zoek recursief in de huidige map en geef een lijst met passende bestanden
- `ugrep -Z3 "zoekpatroon"` : zoek "fuzzy" met maximaal 3 extra, missende of verwisselende karakters in het patroon
- `ugrep -z "zoekpatroon"` : doorzoek ook gecomprimeerde bestanden, zip- en `.tar`-archieven recursief
- `ugrep -g "glob_patroon" "zoekpatroon"` : zoek alleen naar bestanden met namen die overeenkomen met een specifiek `glob`-patroon
- `ugrep -t cpp "zoekpatroon"` : zoek alleen passende bestanden van het type C++ (gebruik `--file-type=list` voor een lijst van typenamen)

## Links
- More information: https://github.com/Genivia/ugrep#man-page
