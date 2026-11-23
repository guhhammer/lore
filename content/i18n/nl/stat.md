# stat

Toon bestands- en bestandssysteeminformatie.

## Usage
- `stat pad/naar/bestand` : toon eigenschappen van een specifiek bestand zoals grootte, permissies, aanmaak- en toegangsdatums en meer
- `stat -t pad/naar/bestand` : toon eigenschappen van een specifiek bestand, waarbij alleen de ruwe resultaatgegevens zonder labels worden getoond
- `stat -f pad/naar/bestand` : toon informatie over het bestandssysteem waar een specifiek bestand zich bevindt
- `stat -c "%a %n" pad/naar/bestand` : toon alleen octale bestandspermissies
- `stat -c "%U %G" pad/naar/bestand` : toon de eigenaar en groep van een specifiek bestand
- `stat -c "%s %n" pad/naar/bestand` : toon de grootte van een specifiek bestand in bytes

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/stat-invocation.html
