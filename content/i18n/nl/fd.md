# fd

Vind items in het bestandssysteem.

## Usage
- `fd "regex"` : vind recursief bestanden die overeenkomen met een specifiek patroon in de huidige map
- `fd "regex" pad/naar/map` : vind bestanden in een specifieke map
- `fd -e txt` : vind bestanden met een specifieke extensie
- `fd "regex" -t d` : vind alleen mappen die overeenkomen met een specifiek patroon
- `fd "regex" -HI` : neem genegeerde en verborgen bestanden op in de zoekopdracht
- `fd "regex" -E glob` : sluit bestanden uit die overeenkomen met een specifiek `glob`-patroon
- `fd "regex" -x commando` : voer een commando uit op elk zoekresultaat
- `fd "regex" -d 1` : vind bestanden alleen in de huidige map

## Links
- More information: https://github.com/sharkdp/fd#command-line-options
