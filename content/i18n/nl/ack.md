# ack

Een zoektool zoals `grep`, geoptimaliseerd voor ontwikkelaars.

## Usage
- `ack "zoekpatroon"` : zoek recursief naar bestanden met een tekenreeks of `regex` in de huidige map
- `ack -i "zoekpatroon"` : zoek naar een niet-hoofdlettergevoelig patroon
- `ack -o "zoekpatroon"` : zoek naar lijnen die overeenkomen met een patroon en druk alleen de overeenkomende tekst af en niet de rest van de lijn
- `ack -t ruby "zoekpatroon"` : beperk het zoeken tot bestanden van een specifiek type
- `ack -t noruby "zoekpatroon"` : zoek niet in bestanden van een specifiek type
- `ack -c -h "zoekpatroon"` : tel het totaal aantal gevonden matches
- `ack -c -l "zoekpatroon"` : toon alleen voor elk bestand de bestandsnamen en het aantal overeenkomsten
- `ack --help-types` : maak een lijst van alle waarden die kunnen worden gebruikt met `--type`

## Links
- More information: https://beyondgrep.com/documentation/
