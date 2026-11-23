# chmod

Verander de toegangstoestemmingen van een bestand of map.

## Usage
- `chmod u+x pad/naar/bestand` : geef de gebruiker (user) die eigenaar is van een bestand het recht om dit uit te voeren (execute)
- `chmod u+rw pad/naar/bestand_of_map` : geef de gebruiker (user) het recht om een bestand of map te lezen (read) en schrijven (write)
- `chmod g-x,o-x pad/naar/bestand` : haal uitvoertoestemming (execute) voor een bestand weg van de groep en anderen (others)
- `chmod a+rx pad/naar/bestand` : geef alle gebruikers toegang om een bestand te lezen (read) en uitvoertoestemming (execute)
- `chmod o=g pad/naar/bestand` : geef anderen (others) die niet in de groep van de eigenaar zitten, dezelfde rechten als de groep
- `chmod o= pad/naar/bestand` : haal alle rechten van de anderen (others) weg
- `chmod 644 pad/naar/bestand` : stel de rechten in met octale notatie
- `chmod -R a+rX pad/naar/map` : geef recursief alle gebruikers (all users) toegang om bestanden te lezen (read) en uitvoertoestemming (eXecute) voor bestanden die al ten minste één uitvoertoestemming hebben en voor alle onderliggende mappen in een map
