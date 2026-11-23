# mv

Verplaats of hernoem bestanden en mappen.

## Usage
- `mv pad/naar/bron pad/naar/doel` : hernoem een bestand of map als het doel geen bestaande map is
- `mv pad/naar/bron pad/naar/bestaande_map` : verplaats een bestand of map naar een bestaande map
- `mv pad/naar/bron1 pad/naar/bron2 ... pad/naar/bestaande_map` : verplaats meerdere bestanden naar een bestaande map, waarbij de bestandsnamen ongewijzigd blijven
- `mv -f pad/naar/bron pad/naar/doel` : vraag niet om bevestiging voordat bestaande bestanden worden overschreven
- `mv -i pad/naar/bron pad/naar/doel` : vraag om bevestiging interactief voordat bestaande bestanden worden overschreven, ongeacht de bestandsrechten
- `mv -n pad/naar/bron pad/naar/doel` : overschrijf geen bestaande bestanden op de doelbestemming
- `mv -v pad/naar/bron pad/naar/doel` : verplaats bestanden in verbose-modus, waarbij de bestanden worden getoond nadat ze zijn verplaatst
- `find /var/log -type f -name '*.log' -print0 | xargs -0 mv -t pad/naar/doel_map` : specificeer de doelmap zodat je externe tools kunt gebruiken om verplaatsbare bestanden te verzamelen

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/mv-invocation.html
