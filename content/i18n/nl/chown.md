# chown

Verander de eigenaar en groep van bestanden en mappen.

## Usage
- `sudo chown gebruiker pad/naar/bestand_of_map` : verander de eigenaar van een bestand/map
- `sudo chown gebruiker:groep pad/naar/bestand_of_map` : verander de eigenaar en groep van een bestand of map
- `sudo chown user: pad/naar/bestand_of_map` : verander de eigenaar en groep zodat beiden de naam `user` krijgen
- `chown :groep pad/naar/bestand_of_map` : verander de groep van een bestand naar een groep waar de huidige gebruiker toe behoort
- `sudo chown -R gebruiker pad/naar/map` : verander recursief de eigenaar van een map en alle inhoud
- `sudo chown -h gebruiker pad/naar/symlink` : verander de eigenaar van een symbolische link
- `sudo chown --reference pad/naar/referentiebestand pad/naar/bestand_of_map` : verander de eigenaar van een bestand of map naar dezelfde als een referentiebestand
