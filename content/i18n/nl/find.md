# find

Vind bestanden of mappen onder een mappenboom, recursief.

## Usage
- `find pad/naar/map -name '*.ext'` : vind bestanden op basis van extensie
- `find pad/naar/map -path '*/path/*/*.ext' -or -name '*patroon*'` : vind bestanden die overeenkomen met meerdere pad-/naampatronen
- `find pad/naar/map -type d -iname '*lib*'` : vind mappen die overeenkomen met een gegeven naam, hoofdletterongevoelig
- `find pad/naar/map -name '*.py' -not -path '*/site-packages/*'` : vind bestanden die overeenkomen met een gegeven patroon, met uitsluiting van specifieke paden
- `find pad/naar/map -maxdepth 1 -size +500k -size -10M` : vind bestanden die overeenkomen met een gegeven groottebereik, waarbij de recursieve diepte beperkt is tot "1"
- `find pad/naar/map -name '*.ext' -exec wc -l {} \;` : voer een commando uit voor elk bestand (gebruik `{}` binnen het commando om toegang te krijgen tot de bestandsnaam)
- `find pad/naar/map -daystart -mtime -1 -exec tar -cvf archief.tar {} \+` : vind alle bestanden die vandaag zijn gewijzigd en geef de resultaten door aan een enkel commando als argumenten
- `find pad/naar/map -type f|d -empty -delete -print` : vind lege bestanden of mappen en verwijder ze uitvoerig

## Links
- More information: https://manned.org/find
