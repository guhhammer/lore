# 7za

Bestandsarchiver met een hoge compressieverhouding. Vergelijkbaar met `7z`, behalve dat het minder bestandstypes ondersteunt, maar platformonafhankelijk is.

## Usage
- `7za a pad/naar/archief.7z pad/naar/bestand_of_map` : archiveer een bestand of map
- `7za a pad/naar/versleuteld.7z -pwachtwoord -mhe=on pad/naar/archief.7z` : versleutel een bestaand archief (inclusief bestandsnamen)
- `7za x pad/naar/archief.7z` : extraheer een archief met behoud van de originele mapstructuur
- `7za x pad/naar/archief.7z -opad/naar/uitkomst` : extraheer een archief naar een specifieke map
- `7za x pad/naar/archief.7z -so` : extraheer een archief naar `stdout`
- `7za a -t7z|bzip2|gzip|lzip|tar|... pad/naar/archief.7z pad/naar/bestand_of_map` : archiveer met een specifiek archieftype
- `7za l pad/naar/archief.7z` : geef een lijst met de inhoud van het archiefbestand
- `7za a pad/naar/archief.7z -mx=0|1|3|5|7|9 pad/naar/bestand_of_map` : zet het niveau van compressie (hoger betekent meer compressie, maar langzamer)

## Links
- More information: https://manned.org/7za
