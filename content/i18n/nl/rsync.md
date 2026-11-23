# rsync

Draag bestanden over naar of van een externe host (maar niet tussen twee externe hosts), met als standaard via SSH. Om een pad te specificeren, gebruik `gebruiker@host:pad/naar/bestand_of_map`.

## Usage
- `rsync pad/naar/bron pad/naar/bestemming` : draag een bestand over (gebruik `--dry-run` om de overdracht te simuleren)
- `rsync -a pad/naar/bron pad/naar/bestemming` : gebruik archiefmodus (kopieer recursief mappen, kopieer symlinks zonder ze op te lossen en behoud machtigingen, eigendom en wijzigingstijden)
- `rsync -zvhP pad/naar/bron pad/naar/bestemming` : comprimeer de data wanneer deze worden verzonden naar de bestemming, toon verbose en voor mensen leesbare voortgang, en behoud gedeeltelijk overgedragen bestanden als er een onderbreking is
- `rsync -r --fsync pad/naar/bron pad/naar/bestemming` : kopieer recursief mappen en zorg ervoor dat elk bestand volledig naar de schijf wordt weggeschreven in plaats van in het RAM-geheugen blijft staan
- `rsync -r pad/naar/bron/ pad/naar/bestemming` : draag de inhoud van een map over, maar niet de map zelf
- `rsync -auL pad/naar/bron pad/naar/bestemming` : gebruik archiefmodus, los symlinks op en sla bestanden over die nieuwer zijn op de bestemming
- `rsync -r --delete rsync://host:pad/naar/bron pad/naar/bestemming` : draag een map over van een externe host waarop `rsyncd` draait en verwijder bestanden op de bestemming die niet op de bron bestaan
- `rsync -e 'ssh -p poort' --info=progress2 host:pad/naar/bron pad/naar/bestemming` : draag een bestand over via SSH met een andere poort dan de standaardpoort (22) en toon globale voortgang

## Links
- More information: https://download.samba.org/pub/rsync/rsync.1
