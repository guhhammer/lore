# ausearch

Zoek in het Linux audit logboek naar gebeurtenissen. Onderdeel van het `audit`-pakket.

## Usage
- `sudo ausearch -m avc` : zoek naar alle SELinux AVC-denial gebeurtenissen
- `sudo ausearch -c httpd` : zoek naar gebeurtenissen die verband houden met een specifiek uitvoerbaar bestand
- `sudo ausearch -ui 1000` : zoek naar gebeurtenissen van een specifieke gebruiker
- `sudo ausearch -ts recent` : zoek naar gebeurtenissen in de laatste 10 minuten
- `sudo ausearch -m user_login -sv no` : zoek naar gefaalde inlogpogingen
- `sudo ausearch -f pad/naar/bestand` : zoek naar gebeurtenissen die verband houden met een specifiek bestand
- `sudo ausearch -m avc --raw` : toon resultaten in ruwe vorm voor verdere verwerking

## Links
- More information: https://manned.org/ausearch
