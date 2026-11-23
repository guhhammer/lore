# cp

Copia fitxers i directoris.

## Usage
- `cp camí/al/fitxer_origen.ext camí/al/fitxer_destí.ext` : copia un fitxer a un altre directori
- `cp camí/al/fitxer_origen.ext camí/al/directori` : copia un fitxer a un altre directori, mantenint el nom
- `cp -R camí/al/directori_origen camí/al/directori_destí` : copia recursivament els continguts d'un directori a un altre (si aquest existeix,els continguts es copien dins)
- `cp -vR camí/al/directori_origen camí/al/directori_destí` : copia un directori recursivament, de manera verbosa (mostra els fitxers a mesura que es van copiant)
- `cp -i *.txt camí/al/directori_destí` : copia els fitxers amb extensió `.txt` a una altra ubicació en mode interactiu (demana al usuari abans de sobreescriure un fitxer)
- `cp -L enllaç camí/al/directori_destí` : copia enllaços simbòlics sense mantenir la referència al original

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/cp-invocation.html
