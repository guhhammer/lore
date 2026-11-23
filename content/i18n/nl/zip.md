# zip

Verpak en comprimeer (archiveer) bestanden in een Zip-archief.

## Usage
- `zip -r pad/naar/gecomprimeerd.zip pad/naar/bestand_of_map1 pad/naar/bestand_of_map2 ...` : voeg bestanden/mappen toe aan een specifiek archief
- `zip -d pad/naar/gecomprimeerd.zip pad/naar/bestand_of_map1 pad/naar/bestand_of_map2 ...` : verwijder bestanden/mappen uit een specifiek archief
- `zip -r pad/naar/gecomprimeerd.zip pad/naar/bestand_of_map1 pad/naar/bestand_of_map2 ... -x pad/naar/uitgesloten_bestanden_of_mappen` : archiveer bestanden/mappen waarbij opgegeven bestanden of mappen worden uitgesloten
- `zip -r -0..9 pad/naar/gecomprimeerd.zip pad/naar/bestand_of_map1 pad/naar/bestand_of_map2 ...` : archiveer bestanden/mappen met een specifiek compressieniveau (`0` - het laagste, `9` - het hoogste)
- `zip -re pad/naar/gecomprimeerd.zip pad/naar/bestand_of_map1 pad/naar/bestand_of_map2 ...` : maak een versleuteld archief met een specifiek wachtwoord
- `zip -rs 3g pad/naar/gecomprimeerd.zip pad/naar/bestand_of_map1 pad/naar/bestand_of_map2 ...` : archiveer bestanden/mappen in een multipart split Zip-archief (bijv. 3 GB delen)
- `zip -sf pad/naar/gecomprimeerd.zip` : print de inhoud van een specifiek archief

## Links
- More information: https://manned.org/zip
